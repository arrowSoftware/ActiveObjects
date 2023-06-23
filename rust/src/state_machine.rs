
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

#[derive(Copy, Clone, Debug)]
struct State {}

impl State {
    fn run(&self, event: AOEvent) {}
}

pub trait StateMachine {
    fn new() -> Self;
    fn initialize(&self) {
        
    }
    fn get_current_state(&self) -> State;
    fn set_current_state(&mut self, new_state: State);
    fn transition_to(&mut self, target_state: State) {
        println!("Transition to -> {:?}", target_state);
        self.set_current_state(target_state);
    }
    fn handled(&mut self) {
        self.set_current_state(self.get_current_state());
    }
    fn process_event(&mut self, event: AOEvent) {
        println!("Processing event");
        // Call current state with event
        // If return from current state indicates a transition then
        // execute the exit event on the current state
        // update the current state to the new state from the transition
        // call the new current state with the enter event.
        self.get_current_state().run(event)
    }
}

///////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct MyStateMachine {
    current_state: State,
    previous_state: State,
    boot_state: State,
    idle_state: State
}

impl StateMachine for MyStateMachine {
    fn new() -> MyStateMachine {
        MyStateMachine { 
            current_state: State {

            }, 
            previous_state: State {

            }, 
            boot_state: State {

            }, 
            idle_state: State {

            } 
        }
    }
    fn get_current_state(&self) -> State {
        self.current_state
    }
    fn set_current_state(&mut self, new_state: State) {
        self.current_state = new_state;
    }
}