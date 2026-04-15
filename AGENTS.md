# AGENT.md — BC250 PS_ON Adapter (CH32V003, Rust)

## Project Overview

ATX PSU soft power control adapter for the AMD BC250 mining board.
The BC250 lacks native ATX PS_ON circuitry; this firmware bridges that gap.

### Key Functions

| Function | Description |
|---|---|
| PS_ON control | Drive ATX PS_ON via optocoupler (active LOW at ATX, active HIGH at MCU) |
| HOST_ON sense | Detect BC250 power state via optocoupler (12V → 3.3V isolated) |
| Power button | Debounced momentary button with short / long press detection |
| Force off | 4-second hold → hard cut ATX PS_ON regardless of HOST_ON state |
| LED output | Power status indicator (15mA limited; J1 bypass up to 250mA) |
| BTN_OUT | Route button pulse to BC250 power button solder pads |

---

## Development Environment

| Item | Value |
|---|---|
| Language | Rust (nightly) |
| HAL | [ch32-hal](https://github.com/ch32-rs/ch32-hal) |
| Runtime | `qingke-rt` |
| Target | `riscv32ec-unknown-none-elf` |
| Programmer | WCH-LinkE (probe-rs or wlink) |
| Build | `cargo build --release` |

### Rust Toolchain Notes

- **Nightly is required** — `riscv32ec` (RV32EC with compressed extension) is not yet stable.
- Place `rust-toolchain.toml` in project root to pin nightly version.
- Target spec file `riscv32ec-unknown-none-elf.json` must be present in project root.
- Build with `-Z build-std=core` (configured in `.cargo/config.toml`).

### Cargo.toml essentials

```toml
[dependencies]
ch32-hal  = { git = "https://github.com/ch32-rs/ch32-hal", features = ["ch32v003f4p6"] }
qingke-rt = "0.4"
qingke    = "0.4"
embedded-hal = "1.0"
panic-halt = "1.0"

[profile.release]
lto = true
opt-level = "s"
strip = false
```

> Adjust the `ch32-hal` feature flag to match the actual package:
> `ch32v003f4p6` = TSSOP-20, `ch32v003j4m6` = SOP-8.

---

## Pin Mapping

**See `PINMAP.md` for all pin assignments.**

All pins must be referenced via the `peripherals` struct returned by `hal::init()`.  
Never hardcode register addresses or GPIO port/pin numbers directly.

```rust
// Correct
let ps_on = Output::new(p.PD4, Level::Low, Default::default());

// Wrong — do not do this
unsafe { (*GPIOD::ptr()).outdr.modify(|_, w| w.odr4().set_bit()); }
```

---

## Project Structure

```
bc250-pson/
├── .cargo/
│   └── config.toml               # target triple, runner (probe-rs or wlink)
├── riscv32ec-unknown-none-elf.json
├── rust-toolchain.toml
├── Cargo.toml
├── PINMAP.md                     # authoritative pin assignments
├── AGENT.md                      # this file
└── src/
    ├── main.rs                   # entry point, peripheral init, main loop
    ├── button.rs                 # debounce + short/long press state machine
    ├── power.rs                  # PS_ON / HOST_ON logic, power state machine
    └── led.rs                    # LED output (solid, blink patterns)
```

---

## Firmware Architecture

### Power State Machine (`power.rs`)

```
IDLE
 └─ btn short press ──→ assert PS_ON ──→ POWERING_ON

POWERING_ON
 ├─ HOST_ON HIGH ─────→ RUNNING
 └─ timeout 5s ───────→ de-assert PS_ON ──→ IDLE   (PSU failed to start)

RUNNING
 ├─ btn short press ──→ pulse BTN_OUT ──→ SOFT_OFF
 ├─ btn hold 4s ──────→ force de-assert PS_ON ──→ IDLE
 └─ HOST_ON lost ─────→ glitch filter 500ms
                          still LOW → de-assert PS_ON ──→ IDLE

SOFT_OFF
 ├─ HOST_ON LOW ──────→ de-assert PS_ON ──→ IDLE
 └─ timeout 10s ──────→ de-assert PS_ON ──→ IDLE   (BC250 hung)
```

### Timing Constants (`main.rs`)

```rust
const DEBOUNCE_MS:        u32 =    20;
const FORCE_OFF_MS:       u32 =  4000; // ATX spec — do not change
const WARN_BLINK_MS:      u32 =  3000; // LED fast-blink warning starts before force off
const BTN_OUT_PULSE_MS:   u32 =   200; // pulse width sent to BC250 button pad
const PSOFF_TIMEOUT_MS:   u32 = 10000; // max wait for HOST_ON LOW after soft off request
const POWERON_TIMEOUT_MS: u32 =  5000; // max wait for HOST_ON HIGH after PS_ON assert
const HOST_ON_GLITCH_MS:  u32 =   500; // debounce unexpected HOST_ON drop in RUNNING
```

---

## Signal Logic Reference

| Signal | ATX / BC250 side | MCU pin level | Notes |
|---|---|---|---|
| PS_ON | LOW = PSU ON | HIGH = PSU ON | Optocoupler inverts |
| HOST_ON | 12V = BC250 ON | LOW = BC250 ON | Optocoupler inverts |
| BTN_IN | LOW = pressed | LOW = pressed | Internal pull-up on MCU |
| BTN_OUT | pulse = button press | HIGH pulse | Active HIGH to BC250 pad |
| LED | ON = powered | HIGH = ON | 15mA via onboard resistor |

> Both PS_ON and HOST_ON pass through a PC817 optocoupler.
> The inversion is absorbed into the naming convention — **MCU HIGH always means "active/on"**.
> Do not add extra inversion in code.

---

## Hardware Notes

### PS_ON Output
- MCU pin HIGH → PC817 LED on → collector pulls ATX PS_ON pin to GND → PSU starts.

### HOST_ON Input
- BC250 HOST_ON = 12V when board is running; 0V when off.
- 1 kΩ series resistor on the 12V side of PC817.
- MCU input has internal pull-up; optocoupler pulls it LOW when BC250 is off.
- Net result: MCU sees HIGH when BC250 is ON.

### LED
- Onboard resistor limits current to **15 mA** — suitable for standard 5V/12V backlit buttons.
- Short solder jumper **J1** to bypass resistor for RGB controllers or high-power LEDs.
- **Maximum continuous load with J1 shorted: 250 mA.**

### BTN_OUT
- Connects to BC250 power/reset button solder pads (bottom side of BC250 board).
- Send a short pulse (200 ms) to simulate a momentary button press.
- Do **not** hold HIGH continuously — the BC250 may interpret a long hold as a hard reset.

---

## Rules for AI Agents

1. **Consult `PINMAP.md` before touching any pin assignment.**
2. **Never hardcode pin numbers.** Use only `peripherals.*` struct fields from `hal::init()`.
3. **All timing constants live in `main.rs` as `const u32`.** Do not duplicate or inline them elsewhere.
4. **`FORCE_OFF_MS = 4000` is fixed by ATX specification.** Do not change it without explicit user instruction.
5. **Optocoupler inversion is already accounted for** in the signal logic table. Do not add extra `!` inversions in logic.
6. **`#![no_std]` only.** Do not use `std::`, `Vec`, `String`, `Box`, `println!`, or any heap allocation.
7. **No blocking loops in the main loop.** Use elapsed-time checks against a `SysTick` counter — never `delay_ms()` in state machine code.
8. **Avoid `unwrap()` on fallible operations** without a comment explaining why it cannot fail.
9. **All dependencies must be `no_std` compatible.** Verify before adding any new crate.
10. **probe-rs is the preferred runner.** Configure in `.cargo/config.toml`; do not commit wlink-specific config without noting it.

---

## Flashing

```bash
# With probe-rs (recommended)
cargo run --release

# With wlink
wlink flash --target ch32v003 target/riscv32ec-unknown-none-elf/release/bc250-pson
```

---

## References

- [ch32-hal](https://github.com/ch32-rs/ch32-hal) — unified HAL with Embassy support
- [ch32-rs org](https://github.com/ch32-rs/ch32-rs) — PAC, SVD, tooling overview
- [CH32V003 Rust getting started](https://albertskog.se/ch32v-in-rust/)
- [CH32V003 Datasheet](https://www.wch-ic.com/products/CH32V003.html)
- [BC250 Community Docs](https://github.com/mothenjoyer69/bc250-documentation)
- `PINMAP.md` — pin assignments for this board revision (always check here first)
