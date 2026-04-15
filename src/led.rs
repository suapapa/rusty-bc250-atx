/// LED blink patterns.
#[derive(Clone, Copy, PartialEq)]
pub enum LedPattern {
    Off,
    On,
    SlowBlink, // 500 ms on / 500 ms off
    FastBlink, // 125 ms on / 125 ms off
}

/// Tracks current pattern and blink timing; driven from the main tick loop.
pub struct LedState {
    pattern: LedPattern,
    acc_ms: u32,
    lit: bool,
}

impl LedState {
    pub const fn new() -> Self {
        Self {
            pattern: LedPattern::Off,
            acc_ms: 0,
            lit: false,
        }
    }

    /// Switch to a new pattern; resets blink phase.
    pub fn set(&mut self, pattern: LedPattern) {
        if pattern == self.pattern {
            return;
        }
        self.pattern = pattern;
        self.acc_ms = 0;
        self.lit = !matches!(pattern, LedPattern::Off);
    }

    /// Call once per tick. Returns `true` if the LED pin should be HIGH this tick.
    pub fn update(&mut self, tick_ms: u32) -> bool {
        match self.pattern {
            LedPattern::Off => false,
            LedPattern::On => true,
            LedPattern::SlowBlink => {
                self.acc_ms += tick_ms;
                if self.acc_ms >= 500 {
                    self.acc_ms = 0;
                    self.lit = !self.lit;
                }
                self.lit
            }
            LedPattern::FastBlink => {
                self.acc_ms += tick_ms;
                if self.acc_ms >= 125 {
                    self.acc_ms = 0;
                    self.lit = !self.lit;
                }
                self.lit
            }
        }
    }
}
