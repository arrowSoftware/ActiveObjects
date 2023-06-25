mod state_machine;
mod active_object;
use state_machine::{AOEvent, AOSignal, State};
use active_object::{ActiveObject};

//enum MySignals {
//    AOSignal(AOSignal),
//    BootCompleteSig
//}

#[derive(Clone)]
struct IdleState {
}

impl IdleState {
    fn new() -> IdleState {
        println!("idle_state::new");
        IdleState { 

         }
    }
}

impl State for IdleState {
    fn run(&self, event: AOEvent) {
        println!("idle_state::run {:?}", event);
        match event.signal {
            AOSignal::AoEnterSig => {
                println!("IdleState::run::Enter event");
            }
            AOSignal::AoExitSig => {
                println!("IdleState::run::Exit event");
            }
            _ => {
                println!("IdleState::rune::Default signal handler");
            }
        }
    }
}

fn main() {
    let mut active_object : ActiveObject = ActiveObject::new();
    let idle_state: Box<IdleState> = Box::new(IdleState::new());
    active_object.initialize(idle_state);
    active_object.start();
    active_object.stop();
}
