use std::sync::{Arc, Mutex};
mod state_machine;
mod active_object;
use state_machine::{AOEvent, AOSignal, State, Action};
use active_object::{ActiveObject};

//enum MySignals {
//    AOSignal(AOSignal),
//    BootCompleteSig
//}

#[derive(Clone)]
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
    fn run(&self, event: AOEvent) -> Action {
        let mut ret: Action;
        println!("BootState::run {:?}", event);
        match event.signal {
            AOSignal::AoEnterSig => {
                println!("BootState::run::Enter event");
                // TODO how to post to the Active Object?
                // ActiveObject::post(AOEvent { signal: AoTestSig })
                ret = Action::Handled;
            }
            AOSignal::AoTestSig => {
                println!("BootState::run::Test event");
                ret = Action::TransitionTo(new_state!(IdleState::new()));
            }
            AOSignal::AoExitSig => {
                println!("BootState::run::Exit event");
                ret = Action::Handled;
            }
            _ => {
                println!("BootState::rune::Default signal handler");
                ret = Action::Handled;
            }
        }
        ret
    }
}

#[derive(Clone)]
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
    fn run(&self, event: AOEvent) -> Action {
        let mut ret: Action;
        println!("IdleState::run {:?}", event);
        match event.signal {
            AOSignal::AoEnterSig => {
                println!("IdleState::run::Enter event");
                ret = Action::Handled;
            }
            AOSignal::AoExitSig => {
                println!("IdleState::run::Exit event");
                ret = Action::Handled;
            }
            _ => {
                println!("IdleState::rune::Default signal handler");
                ret = Action::Handled;
            }
        }
        ret
    }
}

fn main() {
    let active_object : ActiveObject = ActiveObject::new();
    let boot_state: Arc<Mutex<BootState>> = new_state!(BootState::new());
    active_object.initialize(boot_state);
    active_object.start();
    active_object.stop();
}
