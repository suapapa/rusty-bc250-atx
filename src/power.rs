use embassy_time::Instant;

/// ATX / BC250 power states.
#[derive(Clone, Copy, PartialEq)]
pub enum PowerState {
    /// PSU and BC250 are off; waiting for the user to press the power button.
    Idle,
    /// PS_ON asserted; waiting for HOST_ON to rise (BC250 starting up).
    PoweringOn,
    /// BC250 is running normally.
    Running,
}

/// Tracks the current state and per-state timing.
pub struct PowerController {
    pub state: PowerState,
    entered_at: Instant,
    /// Glitch filter: records when HOST_ON first dropped while in Running state.
    pub host_on_lost_at: Option<Instant>,
}

impl PowerController {
    pub fn new() -> Self {
        Self {
            state: PowerState::Idle,
            entered_at: Instant::now(),
            host_on_lost_at: None,
        }
    }

    /// Transition to `new_state`, resetting all per-state timers.
    pub fn transition(&mut self, new_state: PowerState) {
        self.state = new_state;
        self.entered_at = Instant::now();
        self.host_on_lost_at = None;
    }

    /// Milliseconds elapsed since the last state transition.
    pub fn elapsed_ms(&self) -> u64 {
        self.entered_at.elapsed().as_millis()
    }
}
