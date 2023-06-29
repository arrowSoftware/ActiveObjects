use std::sync::{Arc, Mutex};

use crate::action::Action;
use crate::ao_event::AoEvent;
use crate::ao_comms::AoComms;

/**
 * A dummy psuedo state used to initialize the default state in the state
 * machine when its creted.
 */
#[derive(Clone)]
pub struct PsuedoState {}
impl PsuedoState {
    /**
     * A Constructor for the PsuedoState structure.
     * @return PsuedoState object.
     */
    pub fn new() -> PsuedoState {
        println!("psuedoState::new");
        PsuedoState {}
    }
}

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
}

// Helper type for references the thread State object.
pub type StateT = Arc<Mutex<dyn State + Sync + Send>>;

/**
 * Implement the State trait on the psuedo state.
 */
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
}