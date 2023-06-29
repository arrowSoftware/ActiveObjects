use std::sync::{Arc, Mutex};

use crate::action::Action;
use crate::action::Action::*;
use crate::state::State;
use crate::ao_event::AoEvent;
use crate::ao_signal::AoSignal::*;
use crate::active_object::ActiveObject;
use crate::state_machine::StateMachine;
use crate::ao_comms::AoComms;

//enum MySignals {
//    AoSignal(AoSignal),
//    BootCompleteSig
//}

struct BootState {
}
impl BootState {
    fn new() -> BootState {
        println!("BootState::new");
        BootState {
        }
    }
}

impl State for BootState {
    fn run(&mut self, event: AoEvent, ao_comms: &mut AoComms) -> Action {
        let ret: Action;
        println!("BootState::run {:?}", event);
        match event.signal {
            AoEnterSig => {
                println!("BootState::run::Enter event");
                ao_comms.post(AoEvent { signal: AoTestSig });
                ret = Handled;
            }
            AoTestSig => {
                println!("BootState::run::Test event");
                ret = TransitionTo(Arc::new(Mutex::new(IdleState::new())));
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
impl IdleState {
    fn new() -> IdleState {
        println!("IdleState::new");
        IdleState {
        }
    }
}

impl State for IdleState {
    fn run(&mut self, event: AoEvent, ao_comms: &mut AoComms) -> Action {
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
    let boot_state: Arc<Mutex<BootState>> = Arc::new(Mutex::new(BootState::new()));
    active_object.initialize(boot_state);
    active_object.start();
    //active_object.stop();

    loop {}
}
