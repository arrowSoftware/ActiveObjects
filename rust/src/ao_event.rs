use crate::ao_signal::AoSignal;

#[derive(Debug, Copy, Clone)]
pub struct AoEvent {
    pub signal : AoSignal /* AoSignal used to publish\post this event. */
}

impl AoEvent {
    /**
     * A Constructor for the AoEvent structure.
     * @param signal An AoSignal tied to this event.
     * @return AoEvent object.
     */
    pub fn new(signal: AoSignal) -> AoEvent {
        AoEvent { 
            signal 
        }
    }
}
