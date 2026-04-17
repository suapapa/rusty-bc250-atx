# Bill of Materials — rusty-bc250-atx

| Ref | Description | Value / Part | Package | Qty |
|-----|-------------|--------------|---------|-----|
| MOD1 | MCU module | WeActStudio CH32V003 CoreBoard | — | 1 |
| U2  | Optocoupler — PS_ON | PC817C | DIP-4 / SOP-4 | 1 |
| U3  | Optocoupler — HOST_ON | PC817C | DIP-4 / SOP-4 | 1 |
| R1  | PS_ON optocoupler LED current limit | 1 kΩ | 0402 | 1 |
| R2  | HOST_ON optocoupler LED current limit | 1 kΩ | 0402 | 1 |
| R3  | Power LED current limit (15 mA) | 100 Ω | 0402 | 1 |
| J1  | ATX connector (PS_ON + GND) | 2-pin header | 2.54 mm | 1 |
| J2  | Power LED connector | 2-pin header | 2.54 mm | 1 |
| J3  | HOST_ON input connector | 2-pin header | 2.54 mm | 1 |
| J4  | 5 V SB power input (ATX pin 9) | 2-pin header | 2.54 mm | 1 |

## Notes

- **MOD1** includes the CH32V003F4U6, 3.3 V LDO, decoupling caps, and a WCH-Link-compatible debug header — no discrete regulator or debug connector needed.
- Power the module from ATX 5 V SB via the module's VIN(5V) pin.
- **R3** limits PC4 GPIO current to ~15 mA for standard backlit buttons.
- **U2 / U3** can be any PC817 grade (A–D); grade C or better recommended for reliable switching at reduced drive currents.
