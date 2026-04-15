# Pin Map

| Signal | MCU Pin | Direction | Description |
|--------|---------|-----------|-------------|
| PS_ON | PC3 | OUT | ATX PS_ON control via optocoupler (HIGH = PSU on) |
| BTN_IN | PC2 | IN | Power button input, active-low (internal pull-up) |
| PWR_LED_OUT_P | PC4 | OUT | Power status LED anode (HIGH = on) |
| BTN_OUT | PC5 | OUT | BC250 power-button pad output (HIGH pulse = press) |
| HOST_ON_IN | PC6 | IN | BC250 running sense via optocoupler (LOW = BC250 on) |
| VIN | 5V_SB | PWR | ATX 5 V standby power input |
| GND | GND | PWR | Ground |
| PWR_LED_OUT_N | GND | PWR | Power LED cathode (tied to GND) |
