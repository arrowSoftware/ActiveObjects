use std::thread::{self, JoinHandle};

use crate::ao_event::AoEvent;
use crate::state::StateT;
use crate::state_machine::StateMachine;

// The ActiveObject that handles the event processing loop.
pub struct ActiveObject {
    // boolean flag to exit task and join thread.
    exit: bool,
    // Internal state machine.
    state_machine: StateMachine
}

impl ActiveObject {
    /**
     * Creates the ActiveObject object
     * @return ActiveObject object.
     */
    pub fn new() -> ActiveObject {
        println!("ActiveObject::new");
        ActiveObject {
            exit: false,
            state_machine: StateMachine::new()
        }
    }

    pub fn store_publish_event(&self, event: AoEvent) {

    }

    pub fn get_next_event(&self) AoEvent {
        AoEvent::new(AoEnterSig)
    }

    /**
     * Spawns a new thread for this active object and executes the main task
     * method.
     * @param self ActiveObject instance, consumed on call.
     * @param initial_state The initial state for the state machine to enter.
     */
    pub fn start(mut self, initial_state: StateT) -> JoinHandle<()>{
        println!("ActiveObject::start");
        thread::spawn(move || {
            self.task(initial_state);
        })
    }

    /**
     * Will get the next event from the state machine and execute the
     * process_event method for that event.  Will block until a new event
     * is pushed onto the post queue from the state machine.
     * @param  self ActiveObject instance
     */
    pub fn process_one_event(&mut self) {
        println!("ActiveObject::process_one_event");

        let current_event: AoEvent;

        // Loop until there is an event on the queue
        loop {
            match self.state_machine.get_next_event() {
                // Get the new event and break from the loop to process it.
                Some(e) => {
                    current_event = e;
                    break;
                }
                // remain in the loop until an event is pulled.
                None => {}
            };
        }

        // Process the event from the queue
        self.state_machine.process_event(current_event);
    }

    /**
     * Main task point for the active object thread.
     * @param self ActiveObject instance
     * @param initial_state The initial state for the state machine to enter.
     */
    pub fn task(&mut self, initial_state: StateT) {
        println!("ActiveObject::task");

        // Set the initial state for the state machine and execute the enter
        // event on it.
        self.state_machine.initialize(initial_state);

        // Loop forever, processing events.
        while !self.exit {
            self.process_one_event();
        }
    }
}
