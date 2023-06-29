use crate::ao_signal::AoSignal;

pub struct Timer {
    signal: AoSignal,
    interval: i32,
    start_time: i32
}

impl Timer {
    pub fn new(signal: AoSignal, interval: i32) -> Timer {
        Timer {
            signal,
            interval,
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