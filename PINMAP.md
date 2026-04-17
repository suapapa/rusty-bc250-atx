# Pin Map

| Signal | MCU Pin | Direction | Description |
|--------|---------|-----------|-------------|
| VIN | 5V_SB | PWR | ATX 5 V standby power input |
| GND | GND | PWR | Ground |
| PS_ON | PC3 | OUT | ATX PS_ON control via optocoupler (HIGH = PSU on) |
| HOST_ON_IN | PC0 | IN | BC250 running sense via CPU_FAN 2nd pin (12 V) |
| BTN_IN | PC6 | IN | Power button input, active-low (internal pull-up) |
| PWR_LED_OUT_P | PC7 | OUT | Power status LED anode (HIGH = on) |
| PWR_LED_OUT_N | GND | PWR | Power LED cathode (tied to GND) |
