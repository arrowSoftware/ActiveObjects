mod state_machine;
mod active_object;
use state_machine::{StateMachine, AOEvent, AOSignal};
use crate::state_machine::MyStateMachine;

fn main() {
    let mut machine: MyStateMachine = MyStateMachine::new();
}
