use std::thread::{self};

use crate::ao_signal::AoSignal;
use crate::ao_event::AoEvent;
use crate::state::StateT;

use crate::state_machine::StateMachine;

pub struct ActiveObject {
    // boolean flag to exit task and join thread
    exit: bool,
    // internal state machine trait
    state_machine: StateMachine
}

impl ActiveObject {
    pub fn new() -> ActiveObject {
        println!("ActiveObject::new");
        ActiveObject {
            exit: false,
            state_machine: StateMachine::new()
        }
    }
    pub fn start(mut self, initial_state: StateT) {
        println!("ActiveObject::start");
        thread::spawn(move || {
            self.task(initial_state);
        });
    }
    pub fn process_one_event(&mut self) -> bool {
        println!("ActiveObject::process_one_event");
        
        let current_event: AoEvent;

        loop {
            match self.state_machine.get_next_event() {
                Some(e) => {
                    current_event = e;
                    break;
                }
                None => {}
            };
        }

        self.state_machine.process_event(current_event);
        true
    }
    pub fn task(&mut self, initial_state: StateT) {
        println!("ActiveObject::task");

        self.state_machine.initialize(initial_state);

        while !self.exit {
            self.process_one_event();
        }
    }
}
