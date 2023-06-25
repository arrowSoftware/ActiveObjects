use std::sync::{Arc, Mutex};

#[derive(Debug, Copy, Clone)]
pub enum AOSignal {
    AoProbeSig,
    AoEnterSig,
    AoExitSig,
    AoBeginUserSignals
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

//pub trait State : StateClone {
pub trait State {
    fn run(&self, event: AOEvent);
}

//trait StateClone {
//    fn clone_box(&self) -> Arc<Mutex<dyn State>;
//}

//impl<T> StateClone for T
//where 
//    T: 'static + State + Clone,
//{
//    fn clone_box(&self) -> Arc<Mutex<dyn State> {
//        Arc::new(self.clone())
//    }    
//}

//impl Clone for Arc<Mutex<dyn State> {
//    fn clone(&self) -> Arc<Mutex<dyn State> {
//        self.clone_box()
//    }
//}


impl State for PsuedoState {
    fn run(&self, event: AOEvent) {
        println!("psuedoState::run {:?}", event);
    } 
}

pub enum Action {
    Handled,
    TransitionTo(Arc<Mutex<dyn State>>)
}

pub struct InternalStateMachine {
    current_state: Arc<Mutex<dyn State  + Sync + Send>>
}

impl InternalStateMachine {
    pub fn new() -> InternalStateMachine {
        InternalStateMachine {
            current_state: Arc::new(Mutex::new(PsuedoState::new()))
        }
    }
}

pub trait StateMachine {
    fn initialize(&mut self, initial_state: Arc<Mutex<dyn State + Sync + Send>>);
    fn get_current_state(&self) ->  Arc<Mutex<dyn State + Sync + Send>>;
    fn set_current_state(&mut self, new_state: Arc<Mutex<dyn State+ Sync + Send>>);
    fn transition_to(&mut self, target_state: Arc<Mutex<dyn State + Sync + Send>>);
    fn handled(&mut self);
    fn process_event(&mut self, event: AOEvent);   
}

impl StateMachine for InternalStateMachine {
    fn initialize(&mut self, initial_state: Arc<Mutex<dyn State + Sync + Send>>) {
        println!("StateMachine::initialize");
        self.set_current_state(initial_state);
    }
    fn get_current_state(&self) ->  Arc<Mutex<dyn State + Sync + Send>> {
        println!("StateMachine::get_current_state");
        self.current_state.clone()
    }
    fn set_current_state(&mut self, new_state: Arc<Mutex<dyn State + Sync + Send>>) {
        println!("StateMachine::set_current_state");
        self.current_state = new_state.clone();
    }
    fn transition_to(&mut self, target_state: Arc<Mutex<dyn State + Sync + Send>>) {
        println!("StateMachine::transition_to");
        self.set_current_state(target_state);
    }
    fn handled(&mut self) {
        println!("StateMachine::handled");
        self.set_current_state(self.get_current_state());
    }
    fn process_event(&mut self, event: AOEvent) {
        println!("StateMachine::process_event");
        // Call current state with event
        // If return from current state indicates a transition then
        // execute the exit event on the current state
        // update the current state to the new state from the transition
        // call the new current state with the enter event.
        self.get_current_state().lock().unwrap().run(event);
    }
}