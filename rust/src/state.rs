use std::sync::{Arc, Mutex};

use crate::action::Action;
use crate::ao_event::AoEvent;

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
     * @return Action enum.
     */
    fn run(&mut self, event: AoEvent) -> Action;
}

// Helper type for references the thread State object.
pub type StateT = Arc<Mutex<dyn State + Sync + Send>>;

// Helper macro for creating a new arc::mutex state trait
// TODO How the fuck do you reference this in other files?!
macro_rules! new_state {
    ($T:expr) => {
        Arc::new(Mutex::new($T))
    };
}
pub(crate) use new_state;

/*
macro_rules! declare_state {
    ($name: ident) => {
        impl $name {
            fn post(&mut self, event: AoEvent) -> bool {
                println!("{} {:?}", concat!(stringify!($name), "::post"), event);
                let mut ret: bool = false;
                let queue_size: usize = self.postQueue.len();
        
                // If the queue is not full. Add the new event to it.
                // TODO 100 should be self.queue_size
                if queue_size < 100 {
                    match self.postQueue.push_back(event) {
                        Ok(_) => {
                            ret = true;
                        }
                        Err(_) => {
                            ret = false;
                        },
                    };
                }
                ret
            }
            fn postUrgent(&mut self, event: AoEvent) -> bool {
                println!("{} {:?}", concat!(stringify!($name), "::postUrgent"), event);
                let mut ret: bool = false;
                let queue_size: usize = self.postQueue.len();
        
                // If the queue is not full. Add the new event to it.
                // TODO 100 should be self.queue_size
                if queue_size < 100 {
                    match self.postQueue.push_front(event) {
                        Ok(_) => {
                            ret = true;
                        }
                        Err(_) => {
                            ret = false;
                        },
                    };
                }
                ret
            }
            fn publish(&mut self, event: AoEvent) -> bool {
                println!("{} {:?}", concat!(stringify!($name), "::publish"), event);
                true
            }
            fn publishUrgent(&mut self, event: AoEvent) -> bool {
                println!("{} {:?}", concat!(stringify!($name), "::publishUrgent"), event);
                true
            }
        }
    };
}
pub(crate) use declare_state;
*/
/**
 * Implement the State trait on the psuedo state.
 */
impl State for PsuedoState {
    /**
     * The run function for the state.
     * @param self PsuedoState state using this trait.
     * @param event The AoEvent to process.
     * @return Action enum.
     */
    fn run(&mut self, event: AoEvent) -> Action {
        println!("psuedoState::run {:?}", event);
        /* Just return handled since this state doesnt actually do anything,
           it is just a place holder. */
        Action::Handled
    } 
}
