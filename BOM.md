# Bill of Materials — ch32-bc250-atx

| Ref | Description | Value / Part | Package | Qty |
|-----|-------------|--------------|---------|-----|
| MOD1 | MCU module | WeActStudio CH32V003 CoreBoard | — | 1 |
| U2  | Optocoupler — PS_ON | PC817C | DIP-4 / SOP-4 | 1 |
| U3  | Optocoupler — HOST_ON | PC817C | DIP-4 / SOP-4 | 1 |
| R1  | PS_ON optocoupler LED current limit | 1 kΩ | 0402 | 1 |
| R2  | HOST_ON optocoupler LED current limit | 1 kΩ | 0402 | 1 |
| R3  | Power LED current limit (15 mA) | 100 Ω | 0402 | 1 |
| SW1 | Power button | Momentary SPST | 6×6 mm THT | 1 |
| J1  | LED resistor bypass jumper | Solder jumper | — | 1 |
| J2  | ATX connector (PS_ON + GND) | 2-pin header | 2.54 mm | 1 |
| J3  | BC250 power-button pad connector | 2-pin header | 2.54 mm | 1 |
| J4  | Power LED connector | 2-pin header | 2.54 mm | 1 |
| J5  | HOST_ON input connector | 2-pin header | 2.54 mm | 1 |
| J6  | 5 V SB power input (ATX pin 9) | 2-pin header | 2.54 mm | 1 |

## Notes

- **MOD1** includes the CH32V003F4U6, 3.3 V LDO, decoupling caps, and a WCH-Link-compatible debug header — no discrete regulator or debug connector needed.
- Power the module from ATX 5 V SB via the module's 5 V pin.
- **R3** limits PC4 GPIO current to ~15 mA for standard backlit buttons. Short **J1** to bypass R3 for RGB controllers or high-current LEDs (max 250 mA continuous).
- **U2 / U3** can be any PC817 grade (A–D); grade C or better recommended for reliable switching at reduced drive currents.
