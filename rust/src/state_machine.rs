#[derive(Copy, Clone, Debug)]
enum AOSignal {
    AoProbeSig,
    AoEnterSig,
    AoExitSig,
    AoMaxSig,
}

#[derive(Copy, Clone, Debug)]
struct AOEvent {
    sig: AOSignal
}

impl AOEvent {
    fn new(signal: AOSignal) -> AOEvent {
        AOEvent { sig: signal }
    }
}

type StateMethod = fn(&mut State, AOEvent) -> StateRtn;

#[derive(Debug)]
struct StateRtn {
    method: StateMethod
}

impl StateRtn {
    fn new(method: StateMethod) -> StateRtn {
        StateRtn { method: method }
    }
}

#[derive(Debug)]
struct State  {
    enter_event: AOEvent,
    exit_event: AOEvent,
    current_state: StateMethod,
    previous_state: StateMethod
}

trait StateMachine {
    fn new() -> Self;
    fn initialize(&mut self, initial: StateMethod);
    fn process_event(&mut self, event: AOEvent);
    fn transition_to(&self, target_state: StateMethod) -> StateRtn;
    fn handled(&self) -> StateRtn;
    fn initial_psuedo_state(&mut self, event: AOEvent) -> StateRtn;
}

impl StateMachine for State {
    fn new() -> State {
        State { 
            enter_event: AOEvent { 
                sig: AOSignal::AoEnterSig 
            }, 
            exit_event: AOEvent { 
                sig: AOSignal::AoExitSig
            }, 
            current_state: State::initial_psuedo_state,
            previous_state:  State::initial_psuedo_state
        }
    }

    fn initial_psuedo_state(&mut self, event: AOEvent) -> StateRtn {
        StateRtn { 
            method: State::initial_psuedo_state
        }
    }

    fn initialize(&mut self, initial: StateMethod) {
        // Set the state to the initial psuedo state.
        self.current_state = initial;
        let ret: StateRtn = (self.current_state)(self, self.enter_event);
        self.current_state = ret.method;
        (self.current_state)(self, self.enter_event);
    }

    fn process_event(&mut self, event: AOEvent) {
        let ret: StateRtn = (self.current_state)(self, event);
        if ret.method != self.current_state {
            (self.current_state)(self, self.exit_event);
            self.current_state = ret.method;
            (self.current_state)(self, self.enter_event);
        }
    }

    fn transition_to(&self, target_state: StateMethod) -> StateRtn {
        StateRtn { 
            method: target_state 
        }
    }

    fn handled(&self)  -> StateRtn {
        StateRtn { 
            method: self.current_state 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait TestStateMachine : StateMachine {
        fn initial_psuedo_state(&mut self, event: AOEvent) -> StateRtn;
        fn initial_state(&mut self, event: AOEvent) -> StateRtn;
    }

    impl TestStateMachine for State {
        fn initial_psuedo_state(&mut self, event: AOEvent) -> StateRtn {
            self.transition_to(TestStateMachine::initial_state)
        }

        fn initial_state(&mut self, event: AOEvent) -> StateRtn {
            self.handled()
        }
    }

    #[test]
    fn test_new() {
        let mut state: State = StateMachine::new();
        //assert!(state.current_state as usize == StateMachine::initial_psuedo_state as usize);
    }

    fn test_initialize() {
        let mut state: State = TestStateMachine::new();
//        state.initialize(state.initial);
    }
}