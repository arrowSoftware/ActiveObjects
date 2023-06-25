#[derive(Debug)]
pub enum AOSignal {
    AoProbeSig,
    AoEnterSig,
    AoExitSig,
    AoBeginUserSignals
}

#[derive(Debug)]
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

pub trait State : StateClone {
    fn run(&self, event: AOEvent);
}

trait StateClone {
    fn clone_box(&self) -> Box<dyn State>;
}

impl<T> StateClone for T
where 
    T: 'static + State + Clone,
{
    fn clone_box(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }    
}

impl Clone for Box<dyn State> {
    fn clone(&self) -> Box<dyn State> {
        self.clone_box()
    }
}

impl State for PsuedoState {
    fn run(&self, event: AOEvent) {
        println!("psuedoState::run {:?}", event);
    } 
}

pub struct InternalStateMachine {
    current_state: Box<dyn State>
}

impl InternalStateMachine {
    pub fn new() -> InternalStateMachine {
        InternalStateMachine {
            current_state: Box::new(PsuedoState::new())
        }
    }
}

pub trait StateMachine {
    fn initialize(&mut self, initial_state: Box<dyn State>);
    fn get_current_state(&self) ->  Box<dyn State>;
    fn set_current_state(&mut self, new_state: Box<dyn State>);
    fn transition_to(&mut self, target_state: Box<dyn State>);
    fn handled(&mut self);
    fn process_event(&mut self, event: AOEvent);   
}

impl StateMachine for InternalStateMachine {
    fn initialize(&mut self, initial_state: Box<dyn State>) {
        println!("StateMachine::initialize");
        self.set_current_state(initial_state);
    }
    fn get_current_state(&self) ->  Box<dyn State> {
        println!("StateMachine::get_current_state");
        self.current_state.clone()
    }
    fn set_current_state(&mut self, new_state: Box<dyn State>) {
        println!("StateMachine::set_current_state");
        self.current_state = new_state.clone();
    }
    fn transition_to(&mut self, target_state: Box<dyn State>) {
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
        self.get_current_state().run(event);
    }
}