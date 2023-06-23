mod state_machine;
mod active_object;
use new_state_machine::{StateMachine, AOEvent, AOSignal};
use crate::new_state_machine::MyStateMachine;
extern crate arraydeque;

fn main() {
    let mut machine: MyStateMachine = MyStateMachine::new();
}
