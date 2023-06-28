use std::sync::{Arc, Mutex};

use crate::action::Action;
use crate::action::Action::*;
use crate::state::{State, new_state};
use crate::ao_event::AoEvent;
use crate::ao_signal::AoSignal::*;
use crate::active_object::ActiveObject;
use crate::state_machine::StateMachine;

//enum MySignals {
//    AoSignal(AoSignal),
//    BootCompleteSig
//}

struct BootState<'a> {
    state_machine: &'a StateMachine
}
//declare_state!(BootState);
impl BootState<'_> {
    fn new(state_machine: &StateMachine) -> BootState {
        println!("BootState::new");
        BootState {
            state_machine: state_machine
        }
    }
}

impl State for BootState<'_> {
    fn run(&mut self, event: AoEvent) -> Action {
        let ret: Action;
        println!("BootState::run {:?}", event);
        match event.signal {
            AoEnterSig => {
                println!("BootState::run::Enter event");
                //self.post(AoEvent { signal: AoTestSig });
                ret = Handled;
            }
            AoTestSig => {
                println!("BootState::run::Test event");
                //ret = TransitionTo(new_state!(IdleState::new(self.state_machine)));
                ret = Handled;
            }
            AoExitSig => {
                println!("BootState::run::Exit event");
                ret = Handled;
            }
            _ => {
                println!("BootState::rune::Default signal handler");
                ret = Handled;
            }
        }
        ret
    }
}

struct IdleState<'a> {
    state_machine: &'a StateMachine
}
//declare_state!(IdleState);
impl IdleState<'_> {
    fn new(state_machine: &StateMachine) -> IdleState {
        println!("IdleState::new");
        IdleState {
            state_machine: state_machine
        }
    }
}

impl State for IdleState<'_> {
    fn run(&mut self, event: AoEvent) -> Action {
        let ret: Action;
        println!("IdleState::run {:?}", event);
        match event.signal {
            AoEnterSig => {
                println!("IdleState::run::Enter event");
                ret = Handled;
            }
            AoExitSig => {
                println!("IdleState::run::Exit event");
                ret = Handled;
            }
            _ => {
                println!("IdleState::rune::Default signal handler");
                ret = Handled;
            }
        }
        ret
    }
}

pub fn run() {
    let mut active_object : ActiveObject = ActiveObject::new();
    let boot_state: Arc<Mutex<BootState>> = Arc::new(Mutex::new(BootState::new(active_object.get_state_machine())));
    //active_object.initialize(boot_state);
    //active_object.start();
    //active_object.stop();

    loop {}
}
