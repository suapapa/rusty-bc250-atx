/// Events emitted by the button state machine.
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonEvent {
    None,
    /// Released before WARN_BLINK_MS — treat as a normal press.
    ShortPress,
    /// Still held at WARN_BLINK_MS — start warning blink; no state change yet.
    WarnHold,
    /// Still held at FORCE_OFF_MS — force power off.
    LongPress,
}

/// Debounce + hold-time state for a single active-low button.
pub struct ButtonState {
    stable_pressed: bool,
    last_raw: bool,
    same_count: u32,
    hold_ticks: u32,
    warn_fired: bool,
    long_fired: bool,
}

impl ButtonState {
    pub const fn new() -> Self {
        Self {
            stable_pressed: false,
            last_raw: false,
            same_count: 0,
            hold_ticks: 0,
            warn_fired: false,
            long_fired: false,
        }
    }

    /// Call once per tick.
    ///
    /// `pin_low` — raw pin reading; `true` means the pin is LOW (button pressed,
    /// active-low with internal pull-up).
    ///
    /// Returns the event that occurred this tick, or `ButtonEvent::None`.
    pub fn update(
        &mut self,
        pin_low: bool,
        debounce_ticks: u32,
        warn_ticks: u32,
        force_ticks: u32,
    ) -> ButtonEvent {
        // Accumulate consecutive identical readings for debounce.
        if pin_low == self.last_raw {
            self.same_count = self.same_count.saturating_add(1);
        } else {
            self.same_count = 0;
            self.last_raw = pin_low;
        }

        // Accept state change once debounce counter is satisfied.
        if self.same_count >= debounce_ticks && pin_low != self.stable_pressed {
            self.stable_pressed = pin_low;
            self.same_count = 0;

            if !self.stable_pressed {
                // Button released.
                let was_long = self.long_fired;
                self.hold_ticks = 0;
                self.warn_fired = false;
                self.long_fired = false;
                if !was_long {
                    return ButtonEvent::ShortPress;
                }
                return ButtonEvent::None;
            }
        }

        // Track hold duration while button is stably pressed.
        if self.stable_pressed {
            self.hold_ticks = self.hold_ticks.saturating_add(1);

            if !self.long_fired && self.hold_ticks >= force_ticks {
                self.long_fired = true;
                return ButtonEvent::LongPress;
            }

            if !self.warn_fired && self.hold_ticks >= warn_ticks {
                self.warn_fired = true;
                return ButtonEvent::WarnHold;
            }
        }

        ButtonEvent::None
    }

    /// `true` while the button is in the warning-hold window (≥ WARN_BLINK_MS held).
    pub fn is_in_warn(&self) -> bool {
        self.warn_fired && !self.long_fired
    }

    /// `true` while the button is stably pressed (debounced).
    pub fn is_pressed(&self) -> bool {
        self.stable_pressed
    }
}
