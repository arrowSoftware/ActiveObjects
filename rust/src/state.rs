use crate::action::Action;
use crate::ao_event::AoEvent;
use crate::ao_comms::AoComms;
use crate::ao_timer::AoTimer;
use crate::ao_signal::AoSignal;

#[derive(Clone)]
pub struct AoSuper {
    ao_timers: Vec<AoTimer>,
    ao_subscriber_list: Vec<AoSignal>
}

impl AoSuper {
    pub fn new() -> AoSuper {
        AoSuper {
            ao_timers: Vec::new(),
            ao_subscriber_list: Vec::new()
        }
    }
    pub fn add_timer(&mut self, timer: AoTimer) {
        self.ao_timers.push(timer.clone());
    }
    pub fn tick(&mut self) -> Vec<AoSignal> {
        let mut expired_timers: Vec<AoSignal> = vec![];
        for timer in self.ao_timers.iter_mut() {
            if timer.tick() {
                expired_timers.push(timer.signal);
            }
        }
        expired_timers
    }
    pub fn add_subscriber(&mut self, signal: AoSignal) {
        self.ao_subscriber_list.push(signal);
    }
}

pub fn ao_subscribe(ao_super: &mut AoSuper, signal: AoSignal) {
    ao_super.add_subscriber(signal);
}

/**
 * A dummy psuedo state used to initialize the default state in the state
 * machine when its creted.
 */
#[derive(Clone)]
pub struct PsuedoState {
    pub ao_super: AoSuper
}
impl PsuedoState {
    /**
     * A Constructor for the PsuedoState structure.
     * @return PsuedoState object.
     */
    pub fn new() -> PsuedoState {
        println!("psuedoState::new");
        PsuedoState {
            ao_super: AoSuper::new()
        }
    }
}

// State trait for all user states.
pub trait State {
    /**
     * The run function for the state, each new state must implement this function.
     * Use this function to handle incoming events and transition to new states, as well
     * as post/publish events to other active objects.
     * @param self the state using this trait.
     * @param event The AoEvent to process.
     * @param ao_comms The communication structure for the post/publish queues.
     * @return Action enum.
     */
    fn run(&mut self, event: AoEvent, ao_comms: &mut AoComms) -> Action;
    fn get_super(&mut self) -> AoSuper;
}

// Helper type for references the thread State object.
pub type StateT = Box<dyn State + Sync + Send>;

// Implement the State trait on the psuedo state.
impl State for PsuedoState {
    /**
     * The run function for the state.
     * @param self PsuedoState state using this trait.
     * @param event The AoEvent to process.
     * @param ao_comms The communication structure for the post/publish queues.
     * @return Action enum.
     */
    fn run(&mut self, event: AoEvent, _ao_comms: &mut AoComms) -> Action {
        println!("psuedoState::run {:?}", event);
        /* Just return handled since this state doesnt actually do anything,
           it is just a place holder. */
        Action::Handled
    }

    fn get_super(&mut self) -> AoSuper {
        self.ao_super.clone()
    }
}
