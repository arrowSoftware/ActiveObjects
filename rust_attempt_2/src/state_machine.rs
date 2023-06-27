use std::sync::{Arc, Mutex};


use crate::ao_signal::AoSignal;
use crate::ao_event::AoEvent;
use crate::action::Action;
use crate::state::{PsuedoState, StateT};

pub struct InternalStateMachine {
    current_state: StateT
}

impl InternalStateMachine {
    pub fn new() -> InternalStateMachine {
        InternalStateMachine {
            current_state: Arc::new(Mutex::new(PsuedoState::new()))
        }
    }
}

pub trait StateMachine {
    fn initialize(&mut self, initial_state: StateT);
    fn get_current_state(&self) -> StateT;
    fn set_current_state(&mut self, new_state: StateT);
    fn transition_to(&mut self, target_state: StateT);
    fn handled(&mut self);
    fn process_event(&mut self, event: AoEvent);   
}

impl StateMachine for InternalStateMachine {
    fn initialize(&mut self, initial_state: StateT) {
        println!("StateMachine::initialize");
        // Set the state to the initial state.
        self.set_current_state(initial_state);

        // execute the enter event on the initial state.
        match self.get_current_state().lock() {
            Ok(state) => {
                state.run(AoEvent { 
                    signal: AoSignal::AoEnterSig 
                });
            }
            Err(_) => todo!(),
        }
    }
    fn get_current_state(&self) -> StateT {
        println!("StateMachine::get_current_state");
        self.current_state.clone()
    }
    fn set_current_state(&mut self, new_state: StateT) {
        println!("StateMachine::set_current_state");
        self.current_state = new_state.clone();
    }
    fn transition_to(&mut self, target_state: StateT) {
        println!("StateMachine::transition_to");
        self.set_current_state(target_state);
        match self.get_current_state().lock() {
            Ok(state) => {
                state.run(AoEvent { 
                    signal: AoSignal::AoEnterSig 
                });
            }
            Err(_) => todo!(),
        }
    }
    fn handled(&mut self) {
        println!("StateMachine::handled");
        self.set_current_state(self.get_current_state());
    }
    fn process_event(&mut self, event: AoEvent) {
        println!("StateMachine::process_event");

        // Call current state with event
        match self.get_current_state().lock() {
            Ok(state) => {
                match state.run(event) {
                    Action::Handled => {
                        println!("StateMachine::process_event::Handled");
                        self.handled();
                    },
                    Action::TransitionTo(target_state) => {
                        // If return from current state indicates a transition then
                        // execute the exit event on the current state
                        match self.get_current_state().lock() {
                            Ok(state) => {
                                state.run(AoEvent { 
                                    signal: AoSignal::AoExitSig 
                                });

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

pub type StateMachineT = Arc<Mutex<dyn StateMachine + Sync + Send>>;
