
#[derive(Copy, Clone, Debug)]
pub enum AOSignal {
    AoProbeSig,
    AoEnterSig,
    AoExitSig,
    AoMaxSig,
}

#[derive(Copy, Clone, Debug)]
pub struct AOEvent {
    pub sig: AOSignal
}

impl AOEvent {
    fn new(signal: AOSignal) -> AOEvent {
        AOEvent { sig: signal }
    }
}

pub trait State {
    fn new() -> Self;
    fn run(&self, event: AOEvent);
}

pub struct StateMachine {
    pub current_state: State,
    pub previous_state: State
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine { 
            current_state: (), 
            previous_state: () 
        }
    }
    pub fn initialize(&self) {
        !todo!()        
    }
    pub fn get_current_state(&self) -> State {
        self.current_state
    }
    pub fn set_current_state(&mut self, new_state: State) {
        self.current_state = new_state;
    }
    pub fn transition_to(&mut self, target_state: State) {
        println!("Transition to -> {:?}", target_state);
        self.set_current_state(target_state);
    }
    pub fn handled(&mut self) {
        self.set_current_state(self.get_current_state());
    }
    pub fn process_event(&mut self, event: AOEvent) {
        println!("Processing event");
        // Call current state with event
        // If return from current state indicates a transition then
        // execute the exit event on the current state
        // update the current state to the new state from the transition
        // call the new current state with the enter event.
        self.get_current_state().run(event);
    }
}
