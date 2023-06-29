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
            current_state: Box::new(PsuedoState::new()),
            post_queue: ArrayDeque::new()
        }
    }
}

impl StateMachine {
    pub fn initialize(&mut self, initial_state: StateT) {
        println!("StateMachine::initialize");
        // Set the state to the initial state.
        self.current_state = initial_state;

        self.process_event(AoEvent {
            signal: AoSignal::AoEnterSig
        });
    }
    pub fn get_next_event(&mut self) -> Option<AoEvent> {
        let mut opt : Option<AoEvent> = None;
        let event: AoEvent;

        if self.post_queue.is_empty() {
            opt = None
        } else {
            match self.post_queue.front() {
                Some(e) => {
                    opt = Some(*e);
                }
                None => {
                    opt = None;
                }
            }
            self.post_queue.pop_front();
        }
        opt
    }
    pub fn transition_to(&mut self, target_state: StateT) {
        println!("StateMachine::transition_to");
        self.current_state = target_state;
        self.current_state.run(AoEvent { signal: AoSignal::AoEnterSig}, &mut AoComms::new(&mut self.post_queue));
    }
    pub fn handled(&mut self) {
        println!("StateMachine::handled");
    }
    pub fn process_event(&mut self, event: AoEvent) {
        println!("StateMachine::process_event {:?}", event);

        // Call current state with event
        match self.current_state.run(event, &mut AoComms::new(&mut self.post_queue)) {
            Action::Handled => {
                println!("StateMachine::process_event::Handled");
                self.handled();
            },
            Action::TransitionTo(target_state) => {
                // If return from current state indicates a transition then
                // execute the exit event on the current state
                match self.current_state.run(AoEvent {signal: AoSignal::AoExitSig}, &mut AoComms::new(&mut self.post_queue)) {
                    Action::Handled => {}
                    Action::TransitionTo(_) => {}
                }

                println!("StateMachine::process_event::TransitionTo");
                self.transition_to(target_state);
            }
        }
    }
}
