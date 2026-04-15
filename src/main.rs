#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Instant, Timer};
use ch32_hal::gpio::{Input, Level, Output, Pull};
use ch32_hal::println;
use panic_halt as _;

mod button;
mod led;
mod power;

use button::{ButtonEvent, ButtonState};
use led::{LedPattern, LedState};
use power::{PowerController, PowerState};

// ── Timing constants (milliseconds) ────────────────────────────────────────
const TICK_MS: u64 = 10;
const DEBOUNCE_MS: u64 = 20;
const FORCE_OFF_MS: u64 = 4000; // ATX spec — do not change
const WARN_BLINK_MS: u64 = 3000; // fast-blink warning starts this many ms before force-off
const BTN_OUT_PULSE_MS: u64 = 200; // pulse width sent to BC250 button pad
const PSOFF_TIMEOUT_MS: u64 = 10_000; // max wait for HOST_ON LOW after soft-off
const POWERON_TIMEOUT_MS: u64 = 5_000; // max wait for HOST_ON HIGH after PS_ON assert
const HOST_ON_GLITCH_MS: u64 = 500; // debounce unexpected HOST_ON drop while running

// Pre-computed tick counts derived from the constants above.
const DEBOUNCE_TICKS: u32 = (DEBOUNCE_MS / TICK_MS) as u32;
const WARN_TICKS: u32 = (WARN_BLINK_MS / TICK_MS) as u32;
const FORCE_TICKS: u32 = (FORCE_OFF_MS / TICK_MS) as u32;

#[embassy_executor::main(entry = "ch32_hal::entry")]
async fn main(_spawner: Spawner) -> ! {
    ch32_hal::debug::SDIPrint::enable();
    let p = ch32_hal::init(ch32_hal::Config::default());

    // ── Outputs ──────────────────────────────────────────────────────────────
    // PC3: PS_ON — MCU HIGH = optocoupler on = ATX PS_ON pulled LOW = PSU starts.
    let mut ps_on = Output::new(p.PC3, Level::Low, Default::default());
    // PC5: BTN_OUT — HIGH pulse simulates a press on the BC250 power-button pad.
    let mut btn_out = Output::new(p.PC5, Level::Low, Default::default());
    // PC4: PWR_LED_OUT_P — HIGH = LED on.
    let mut led_pin = Output::new(p.PC4, Level::Low, Default::default());

    // ── Inputs ───────────────────────────────────────────────────────────────
    // PC6: HOST_ON_IN — HIGH = BC250 is running (per signal-logic table).
    // The PC817 optocoupler inversion is accounted for in the signal table;
    // no extra inversion needed here.
    let host_on_pin = Input::new(p.PC6, Pull::Up);
    // PC2: BTN_IN — LOW = pressed (active-low with internal pull-up).
    let btn_in_pin = Input::new(p.PC2, Pull::Up);

    // ── State ────────────────────────────────────────────────────────────────
    let mut btn = ButtonState::new();
    let mut led = LedState::new();
    let mut pwr = PowerController::new();
    // Countdown ticks remaining for the BTN_OUT pulse.
    let mut btn_out_ticks: u32 = 0;

    println!("bc250-atx: started");

    loop {
        Timer::after_millis(TICK_MS).await;

        // Read raw inputs.
        let pin_low = btn_in_pin.is_low(); // true = button pressed
        let host_on = host_on_pin.is_low(); // true = BC250 running (optocoupler inverts)

        // Advance button state machine.
        let btn_event = btn.update(pin_low, DEBOUNCE_TICKS, WARN_TICKS, FORCE_TICKS);

        // Drive BTN_OUT pulse.
        if btn_out_ticks > 0 {
            btn_out_ticks -= 1;
            if btn_out_ticks == 0 {
                btn_out.set_low();
            }
        }

        // ── Power state machine ───────────────────────────────────────────────
        match pwr.state {
            PowerState::Idle => {
                led.set(LedPattern::Off);
                if btn_event == ButtonEvent::ShortPress {
                    ps_on.set_high();
                    pwr.transition(PowerState::PoweringOn);
                    println!("IDLE -> POWERING_ON");
                }
            }

            PowerState::PoweringOn => {
                led.set(LedPattern::SlowBlink);
                if host_on {
                    pwr.transition(PowerState::Running);
                    println!("POWERING_ON -> RUNNING");
                } else if pwr.elapsed_ms() >= POWERON_TIMEOUT_MS {
                    ps_on.set_low();
                    pwr.transition(PowerState::Idle);
                    println!("POWERING_ON timeout -> IDLE");
                }
            }

            PowerState::Running => {
                // HOST_ON glitch filter: only de-assert PS_ON after HOST_ON has
                // been continuously LOW for HOST_ON_GLITCH_MS.
                if !host_on {
                    if pwr.host_on_lost_at.is_none() {
                        pwr.host_on_lost_at = Some(Instant::now());
                    } else {
                        let lost_ms = pwr
                            .host_on_lost_at
                            .unwrap() // safe: checked is_none above
                            .elapsed()
                            .as_millis();
                        if lost_ms >= HOST_ON_GLITCH_MS {
                            ps_on.set_low();
                            pwr.transition(PowerState::Idle);
                            println!("RUNNING HOST_ON lost -> IDLE");
                        }
                    }
                } else {
                    pwr.host_on_lost_at = None;
                }

                // LED: fast-blink while holding through warn window, solid otherwise.
                if btn.is_in_warn() {
                    led.set(LedPattern::FastBlink);
                } else {
                    led.set(LedPattern::On);
                }

                match btn_event {
                    ButtonEvent::ShortPress => {
                        // Soft power off: pulse BTN_OUT to tell BC250 to shut down.
                        btn_out.set_high();
                        btn_out_ticks = (BTN_OUT_PULSE_MS / TICK_MS) as u32;
                        pwr.transition(PowerState::SoftOff);
                        println!("RUNNING btn short -> SOFT_OFF");
                    }
                    ButtonEvent::LongPress => {
                        // Force off: immediately cut PS_ON.
                        ps_on.set_low();
                        pwr.transition(PowerState::Idle);
                        println!("RUNNING force off -> IDLE");
                    }
                    _ => {}
                }
            }

            PowerState::SoftOff => {
                led.set(LedPattern::SlowBlink);
                if !host_on {
                    ps_on.set_low();
                    pwr.transition(PowerState::Idle);
                    println!("SOFT_OFF HOST_ON low -> IDLE");
                } else if pwr.elapsed_ms() >= PSOFF_TIMEOUT_MS {
                    // BC250 did not shut down in time; force cut.
                    ps_on.set_low();
                    pwr.transition(PowerState::Idle);
                    println!("SOFT_OFF timeout -> IDLE");
                }
            }
        }

        // Apply LED pattern output.
        if led.update(TICK_MS as u32) {
            led_pin.set_high();
        } else {
            led_pin.set_low();
        }
    }
}
