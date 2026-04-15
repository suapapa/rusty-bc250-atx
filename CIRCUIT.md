# Circuit Description

## PS_ON Control (PC3 → ATX PS_ON)

```
PC3 ──[R1 1kΩ]──→ U2 LED+
                   U2 LED- ──→ GND

U2 Collector ──→ ATX PS_ON (pin 16)
U2 Emitter   ──→ ATX GND   (pin 15)
```

PC3 HIGH → U2 on → ATX PS_ON pulled to GND → PSU starts.

---

## HOST_ON Sense (BC250 → PC6 / HOST_ON_IN)

```
HOST_ON (12 V) ──[R2 1kΩ]──→ U3 LED+
                               U3 LED- ──→ GND

PC6 (internal pull-up) ──→ U3 Collector ──→ PC6 (HOST_ON_IN)
                            U3 Emitter   ──→ GND
```

BC250 ON (12 V) → U3 on → PC6 pulled LOW.  
BC250 OFF (0 V) → U3 off → internal pull-up → PC6 HIGH.

---

## Power Button Input (PC2 / BTN_IN)

```
PC2 (internal pull-up) ──┬── SW1 ──→ GND
```

Button pressed → PC2 LOW (active-low).

---

## BC250 Button Output (PC5 / BTN_OUT)

```
PC5 ──→ BC250 power-button solder pad (board underside)
GND ──→ BC250 power-button solder pad GND
```

PC5 HIGH pulse (200 ms) simulates a momentary button press on the BC250.

---

## Power LED (PC4 / PWR_LED_OUT_P)

```
PC4 ──[R3 100Ω]──[J1]──→ LED+ (PWR_LED_OUT_P)
                           LED- ──→ GND (PWR_LED_OUT_N)
```

R3 limits current to ~15 mA for standard backlit buttons.  
Short solder jumper J1 to bypass R3 for high-current loads (max 250 mA).
