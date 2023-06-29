use std::sync::{Arc, Mutex};
use arraydeque::{ArrayDeque, CapacityError};

use crate::ao_signal::AoSignal;
use crate::ao_event::AoEvent;
use crate::action::Action;
use crate::state::{PsuedoState, StateT};
use crate::ao_comms::AoComms;

pub struct StateMachine {
    pub current_state: StateT,
    post_queue: ArrayDeque<AoEvent, 100>
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            current_state: Arc::new(Mutex::new(PsuedoState::new())),
            post_queue: ArrayDeque::new()
        }
    }
}

impl StateMachine {
    pub fn initialize(&mut self, initial_state: StateT) {
        println!("StateMachine::initialize");
        // Set the state to the initial state.
        self.set_current_state(initial_state);
    }
    pub fn get_current_state(&self) -> StateT {
        println!("StateMachine::get_current_state");
        self.current_state.clone()
    }
    pub fn set_current_state(&mut self, new_state: StateT) {
        println!("StateMachine::set_current_state");
        self.current_state = new_state.clone();
    }
    pub fn transition_to(&mut self, target_state: StateT) {
        println!("StateMachine::transition_to");
        self.set_current_state(target_state);
        match self.get_current_state().lock() {
            Ok(mut state) => {
                state.run(AoEvent {
                    signal: AoSignal::AoEnterSig
                }, &mut AoComms::new(&self.post_queue));
            }
            Err(_) => todo!(),
        }
    }
    pub fn handled(&mut self) {
        println!("StateMachine::handled");
        self.set_current_state(self.get_current_state());
    }
    pub fn process_event(&mut self, event: AoEvent) {
        println!("StateMachine::process_event {:?}", event);

        // Call current state with event
        match self.get_current_state().lock() {
            Ok(mut state) => {
                match state.run(event, &mut AoComms::new(&self.post_queue)) {
                    Action::Handled => {
                        println!("StateMachine::process_event::Handled");
                        self.handled();
                    },
                    Action::TransitionTo(target_state) => {
                        // If return from current state indicates a transition then
                        // execute the exit event on the current state
                        match self.get_current_state().lock() {
                            Ok(mut state) => {
                                state.run(AoEvent {
                                    signal: AoSignal::AoExitSig
                                }, &mut AoComms::new(&self.post_queue));

                                println!("StateMachine::process_event::TransitionTo");
                                self.transition_to(target_state);
                            }
                            Err(_) => todo!(),
                        }
                    }
                }
            }
            Err(_) => todo!(),
        }
    }
}
