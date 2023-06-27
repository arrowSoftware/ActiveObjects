use std::sync::{Arc, Mutex};

mod state_machine;
mod active_object;
mod AoSignal;
mod AoEvent;
mod State;
mod Action;

//enum MySignals {
//    AoSignal(AoSignal),
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

impl State::State for BootState {
    fn run(&self, event: AoEvent::AoEvent) -> Action::Action {
        let mut ret: Action::Action;
        println!("BootState::run {:?}", event);
        match event.signal {
            AoSignal::AoSignal::AoEnterSig => {
                println!("BootState::run::Enter event");
                // TODO how to post to the Active Object?
                // ActiveObject::post(AoEvent { signal: AoTestSig })
                ret = Action::Action::Handled;
            }
            AoSignal::AoSignal::AoTestSig => {
                println!("BootState::run::Test event");
                ret = Action::Action::TransitionTo(Arc::new(Mutex::new(IdleState::new())));
            }
            AoSignal::AoSignal::AoExitSig => {
                println!("BootState::run::Exit event");
                ret = Action::Action::Handled;
            }
            _ => {
                println!("BootState::rune::Default signal handler");
                ret = Action::Action::Handled;
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

impl State::State for IdleState {
    fn run(&self, event: AoEvent::AoEvent) -> Action::Action {
        let mut ret: Action::Action;
        println!("IdleState::run {:?}", event);
        match event.signal {
            AoSignal::AoSignal::AoEnterSig => {
                println!("IdleState::run::Enter event");
                ret = Action::Action::Handled;
            }
            AoSignal::AoSignal::AoExitSig => {
                println!("IdleState::run::Exit event");
                ret = Action::Action::Handled;
            }
            _ => {
                println!("IdleState::rune::Default signal handler");
                ret = Action::Action::Handled;
            }
        }
        ret
    }
}

fn main() {
    let active_object : active_object::ActiveObject = active_object::ActiveObject::new();
    let boot_state: Arc<Mutex<BootState>> = Arc::new(Mutex::new(BootState::new()));
    active_object.initialize(boot_state);
    active_object.start();
    active_object.stop();
}
