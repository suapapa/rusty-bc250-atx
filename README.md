# rusty-bc250-atx

ATX PSU soft-power adapter for the AMD BC250 mining board.
The BC250 lacks native ATX PS_ON circuitry; this firmware bridges that gap using a CH32V003 microcontroller.

## Features

| Function | Description |
|----------|-------------|
| PS_ON control | Drive ATX PS_ON via optocoupler — active LOW at ATX connector, active HIGH at MCU |
| HOST_ON sense | Detect BC250 running state via optocoupler (12 V → 3.3 V isolated) |
| Power button | Debounced momentary button with short-press / long-press detection |
| Force off | 4-second hold → hard-cut ATX PS_ON regardless of HOST_ON state |
| LED output | Power status indicator (15 mA limited; J1 bypass up to 250 mA) |

## Power State Machine

```
IDLE
 └─ short press ──────────→ assert PS_ON ──→ POWERING_ON

POWERING_ON
 ├─ HOST_ON low ──────────→ RUNNING
 └─ timeout 5 s ──────────→ de-assert PS_ON ──→ IDLE   (PSU failed to start)

RUNNING
 ├─ hold 3 s ─────────────→ LED fast-blink warning
 ├─ hold 4 s ─────────────→ force de-assert PS_ON ──→ IDLE
 └─ HOST_ON lost 500 ms ──→ de-assert PS_ON ──→ IDLE
```

## Hardware

- **MCU** — CH32V003F4U6 (RISC-V RV32EC, TSSOP-20)
- **Programmer** — WCH-LinkE via SWIO (PD1)
- See [PINMAP.md](PINMAP.md) for pin assignments
- See [CIRCUIT.md](CIRCUIT.md) for circuit diagrams
- See [BOM.md](BOM.md) for the full bill of materials

## Building

Rust nightly toolchain is required (`riscv32ec` target is not yet stable).
The correct toolchain is pinned via `rust-toolchain.toml`.

```bash
cargo build --release
```

## Flashing

Install [wlink](https://github.com/ch32-rs/wlink):

```bash
cargo install --git https://github.com/ch32-rs/wlink
```

Connect a WCH-LinkE probe to the board (SWIO → PD1, GND, 3.3 V), then:

```bash
cargo run --release
```

Serial debug output (SDI print) is enabled automatically and streamed by `wlink`.

## License

Licensed under the [MIT License](LICENSE-MIT).
