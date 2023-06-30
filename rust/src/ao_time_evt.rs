use crate::ao_signal::AoSignal;

pub struct AoTimeEvt {
    signal: AoSignal,
    clock_ticks: i32,
    start_time: i32
}

impl AoTimeEvt {
    pub fn new(signal: AoSignal, clock_ticks: i32) -> AoTimeEvt {
        AoTimeEvt {
            signal,
            clock_ticks,
            start_time: 0
        }
    }
    pub fn arm(&self) {

    }
    pub fn rearm(&self) {

    }
    pub fn disarm(&self) {
        
    }
}