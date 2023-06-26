use std::sync::{Arc, Mutex};

#[derive(Debug, Copy, Clone)]
pub enum AOSignal {
    AoProbeSig,
    AoEnterSig,
    AoExitSig,
    AoBeginUserSignals,
    AoTestSig
}

#[derive(Debug, Copy, Clone)]
pub struct AOEvent {
    pub signal : AOSignal
}

impl AOEvent {
    pub fn new(signal: AOSignal) -> AOEvent {
        AOEvent { signal: signal }
    }
}

#[derive(Copy, Clone)]
struct PsuedoState {
}

impl PsuedoState {
    fn new() -> PsuedoState {
        println!("psuedoState::new");
        PsuedoState { 

        }
    }
}

pub trait State {
    fn run(&self, event: AOEvent) -> Action;
}

pub type StateT = Arc<Mutex<dyn State + Sync + Send>>;

#[macro_export]
macro_rules! new_state {
    ($T:expr) => {
        Arc::new(Mutex::new($T))
    };
}

pub enum Action {
    Handled,
    TransitionTo(StateT)
}

impl State for PsuedoState {
    fn run(&self, event: AOEvent) -> Action {
        println!("psuedoState::run {:?}", event);
        Action::Handled
    } 
}

pub struct InternalStateMachine {
    current_state: StateT
}

impl InternalStateMachine {
    pub fn new() -> InternalStateMachine {
        InternalStateMachine {
            //current_state: Arc::new(Mutex::new(PsuedoState::new()))
            current_state: new_state!(PsuedoState::new())
        }
    }
}

pub trait StateMachine {
    fn initialize(&mut self, initial_state: StateT);
    fn get_current_state(&self) -> StateT;
    fn set_current_state(&mut self, new_state: StateT);
    fn transition_to(&mut self, target_state: StateT);
    fn handled(&mut self);
    fn process_event(&mut self, event: AOEvent);   
}

impl StateMachine for InternalStateMachine {
    fn initialize(&mut self, initial_state: StateT) {
        println!("StateMachine::initialize");
        // Set the state to the initial state.
        self.set_current_state(initial_state);

        // execute the enter event on the initial state.
        self.get_current_state().lock().unwrap().run(AOEvent { 
            signal: AOSignal::AoEnterSig 
        });

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
        self.get_current_state().lock().unwrap().run(AOEvent { 
            signal: AOSignal::AoEnterSig 
        });
    }
    fn handled(&mut self) {
        println!("StateMachine::handled");
        self.set_current_state(self.get_current_state());
    }
    fn process_event(&mut self, event: AOEvent) {
        println!("StateMachine::process_event");

        // Call current state with event
        match self.get_current_state().lock().unwrap().run(event) {
            Action::Handled => {
                println!("StateMachine::process_event::Handled");
                self.handled();
            },
            Action::TransitionTo(target_state) => {
                // If return from current state indicates a transition then
                // execute the exit event on the current state
                self.get_current_state().lock().unwrap().run(AOEvent { 
                    signal: AOSignal::AoExitSig 
                });
                println!("StateMachine::process_event::TransitionTo");
                self.transition_to(target_state);
            }
        }
    }
}