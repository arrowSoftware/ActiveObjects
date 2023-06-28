use std::sync::{Arc, Mutex};
use arraydeque::ArrayDeque;

use crate::ao_signal::AoSignal;
use crate::ao_event::AoEvent;
use crate::action::Action;
use crate::state::{PsuedoState, StateT};

pub struct StateMachine {
    pub current_state: StateT,
    postQueue: ArrayDeque<AoEvent, 100>,
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            current_state: Arc::new(Mutex::new(PsuedoState::new())),
            postQueue: ArrayDeque::new()
        }
    }
}

//pub trait StateMachine {
//    fn initialize(&mut self, initial_state: StateT);
//    fn get_current_state(&self) -> StateT;
//    fn set_current_state(&mut self, new_state: StateT);
//    fn transition_to(&mut self, target_state: StateT);
//    fn handled(&mut self);
//    fn process_event(&mut self, event: AoEvent);   
//}

impl StateMachine {//for InternalStateMachine {
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
    fn post(&mut self, event: AoEvent) -> bool {
        println!("StateMachine::post {:?}", event);
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
        println!("StateMachine::postUrgent {:?}", event);
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
        println!("StateMachine::publish {:?}", event);
        true
    }
    fn publishUrgent(&mut self, event: AoEvent) -> bool {
        println!("StateMachine::publishUrgent {:?}", event);
        true
    }
    pub fn transition_to(&mut self, target_state: StateT) {
        println!("StateMachine::transition_to");
        self.set_current_state(target_state);
        match self.get_current_state().lock() {
            Ok(mut state) => {
                state.run(AoEvent { 
                    signal: AoSignal::AoEnterSig 
                });
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
                match state.run(event) {
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

//pub type StateMachineT = Arc<Mutex<dyn StateMachine + Sync + Send>>;
