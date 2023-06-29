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

    pub fn initialize(&mut self, state: StateT) {
        self.state_machine.initialize(state);
    }
    pub fn start(mut self) {
        println!("ActiveObject::start");
        thread::spawn(move || {
            self.task();
        });
    }
    pub fn process_one_event(&mut self) -> bool {
        println!("ActiveObject::process_one_event");
        let event: AoEvent;

        let (lock, cvar) = self.state_machine.ao_comms.get_post_queue();
        let mut queue = match lock.lock() {
            Ok(q) => {
                q
            },
            Err(_) => { todo!() }
        };

        while queue.is_empty() {
            //match cvar.wait(queue) {
            //    Ok(_) => {}
            //    Err(_) => todo!(),
            //}
        }

        match queue.front() {
            Some(e) => {
                event = *e;
            }
            None => todo!(),
        }

        queue.pop_front();

        self.state_machine.process_event(event);
        true
    }
    pub fn task(&mut self) {
        println!("ActiveObject::task");

        // execute the enter event on the initial state.
        match self.state_machine.get_current_state().lock() {
            Ok(mut state) => {
                state.run(AoEvent {
                    signal: AoSignal::AoEnterSig
                }, &mut self.state_machine.ao_comms);
            }
            Err(_) => todo!(),
        }

        while !self.exit {
            self.process_one_event();
        }
    }
}
