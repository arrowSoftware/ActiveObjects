use std::sync::{Arc, Mutex};
use arraydeque::ArrayDeque;

use crate::action::Action;
use crate::action::Action::*;
use crate::state::{State, new_state};
use crate::ao_event::AoEvent;
use crate::ao_signal::AoSignal::*;
use crate::active_object::ActiveObject;
use crate::state_machine::StateMachineT;

//enum MySignals {
//    AoSignal(AoSignal),
//    BootCompleteSig
//}

struct BootState {
}
//declare_state!(BootState);
impl BootState {
    fn new() -> BootState {
        println!("BootState::new");
        BootState {

        }
    }
}

impl State for BootState {
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
                ret = TransitionTo(new_state!(IdleState::new()));
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

struct IdleState {
}
//declare_state!(IdleState);
impl IdleState {
    fn new() -> IdleState {
        println!("IdleState::new");
        IdleState {
        }
    }
}

impl State for IdleState {
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
    let active_object : ActiveObject = ActiveObject::new();
    let boot_state: Arc<Mutex<BootState>> = Arc::new(Mutex::new(BootState::new()));
    active_object.initialize(boot_state);
    active_object.start();
//    active_object.stop();

    loop {}
}
