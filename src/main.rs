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
    // PC4: Built-in LED — HIGH = on. On by default; turns off while BTN_IN is held.
    let mut builtin_led = Output::new(p.PC4, Level::High, Default::default());
    // PC7: PWR_LED_OUT_P — HIGH = LED on.
    let mut led_pin = Output::new(p.PC7, Level::Low, Default::default());

    // ── Inputs ───────────────────────────────────────────────────────────────
    // PC0: HOST_ON_IN — LOW = BC250 is running (optocoupler inverts).
    let host_on_pin = Input::new(p.PC0, Pull::Up);
    // PC6: BTN_IN — LOW = pressed (active-low with internal pull-up).
    let btn_in_pin = Input::new(p.PC6, Pull::Up);

    // ── State ────────────────────────────────────────────────────────────────
    let mut btn = ButtonState::new();
    let mut led = LedState::new();
    let mut pwr = PowerController::new();

    println!("bc250-atx: started");

    let mut dbg_tick: u32 = 0;

    loop {
        Timer::after_millis(TICK_MS).await;

        // Read raw inputs.
        let pin_low = btn_in_pin.is_low(); // true = button pressed
        let host_on = host_on_pin.is_low(); // true = BC250 running (optocoupler inverts)

        // DEBUG: print pin_low for the first 20 ticks
        if dbg_tick < 20 {
            println!("dbg tick={} pin_low={}", dbg_tick, pin_low);
            dbg_tick += 1;
        }

        // Advance button state machine.
        let btn_event = btn.update(pin_low, DEBOUNCE_TICKS, WARN_TICKS, FORCE_TICKS);

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

        // Built-in LED: on by default; off while BTN_IN is stably held (debounced).
        if btn.is_pressed() {
            builtin_led.set_low();
        } else {
            builtin_led.set_high();
        }
    }
}
