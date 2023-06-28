use std::sync::{Arc, Mutex, Condvar};
use std::thread::{self};
use arraydeque::ArrayDeque;

use crate::ao_signal::AoSignal;
use crate::ao_event::AoEvent;
use crate::state::StateT;

use crate::state_machine::{
    StateMachineT,
    StateMachine, 
    InternalStateMachine,
};

pub struct ActiveObject {
    // boolean flag to exit task and join thread
    exit: bool,
    // conditional flag
    conditional_var: Condvar,
    // Mutex to lock when accessing the event queue
    mutex: Mutex<bool>,
    // queue contining all incoming events.
    queue: ArrayDeque<AoEvent, 100>,
    // queue size
    queue_size: u32,
    // stack size
    stack_size: usize,
    // internal state machine trait
    state_machine: Arc<Mutex<dyn StateMachine + Sync + Send>>
}

impl ActiveObject {
    pub fn new() -> ActiveObject {
        println!("ActiveObject::new");
        ActiveObject {
            exit: false, 
            //thread_builder: thread::Builder::new().name(name).stack_size(stack_size), 
            conditional_var: Condvar::new(), 
            mutex: Mutex::new(false), 
            queue: ArrayDeque::new(), 
            queue_size: 100,
            stack_size: 100,
            state_machine: Arc::new(Mutex::new(InternalStateMachine::new()))
        }
    }

    pub fn initialize(&self, state: StateT) {
        self.state_machine.lock().unwrap().initialize(state);
    }
    pub fn start(mut self) {
        println!("ActiveObject::start");
        thread::spawn(move || {
            self.task();
        });
    }   
    pub fn process_one_event(&mut self) -> bool {
        println!("ActiveObject::process_one_event");
        let guard: std::sync::MutexGuard<'_, bool>;

        match self.mutex.lock() {
            Ok(g) => {
                guard = g;
            }
            Err(_) => todo!(),
        }

        let event: AoEvent;

        while self.queue.is_empty() {
            //match self.conditional_var.wait(self.mutex.lock().unwrap()) {
            //    Ok(_) => {

            //    }
            //    Err(_) => todo!(),
            //}
        }

        match self.queue.front() {
            Some(e) => {
                event = *e;
            }
            None => todo!(),
        }

        self.queue.pop_front();
        //drop(guard);

        match self.state_machine.lock() {
            Ok(mut sm) => {
                sm.process_event(event);
            }
            Err(_) => todo!(),
        }
        true
    }
    pub fn task(&mut self) {
        println!("ActiveObject::task");

        // execute the enter event on the initial state.
        match self.state_machine.lock().unwrap().get_current_state().lock() {
            Ok(mut state) => {
                state.run(AoEvent { 
                    signal: AoSignal::AoEnterSig 
                });
            }
            Err(_) => todo!(),
        }

        while !self.exit {
            self.process_one_event();
        }
    }
}
