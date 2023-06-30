use crate::ao_signal::AoSignal;
use crate::state::AoSuper;

#[derive(Clone)]
pub struct AoTimer {
    pub signal: AoSignal,
    interval: i32,
    tick_count: i32
}

impl AoTimer {
    pub fn new(ao_super: &mut AoSuper, signal: AoSignal, interval: i32) {
        let ao_timer: AoTimer = AoTimer {
            signal,
            interval,
            tick_count: 0
        };
        ao_super.add_timer(ao_timer);
    }
    pub fn arm(&mut self, nticks: i32, interval: i32) {
        self.tick_count = nticks;
        self.interval = interval;
    }
    pub fn rearm(&mut self, nticks: i32) {
        self.tick_count = nticks;
    }
    pub fn disarm(&mut self) {
        if self.tick_count != 0 {
            self.tick_count = 0;
        }
    }
    pub fn tick(&mut self) -> bool {
        let mut is_expired: bool = false;

        self.tick_count -= 1;

        // Timer is about to expire.
        if self.tick_count == 0 {

            // Periodic timer. Restart tick counter.
            if self.interval != 0 {
                self.tick_count = self.interval;
            } else {
                // One-Shot timer.
                // TODO
            }

            // post event to the active object.
            is_expired = true;
        }
        is_expired
    }
}