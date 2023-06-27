use std::sync::{Arc, Mutex, Condvar};
use std::thread::{self};
use arraydeque::ArrayDeque;

use crate::ao_signal::AoSignal;
use crate::ao_event::AoEvent;
use crate::state::StateT;

use crate::state_machine::{
    StateMachine, 
    InternalStateMachine,
};

struct ActiveObjectInternal {
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

impl ActiveObjectInternal {
    fn initialize(&self) {
        println!("ActiveObjectInternal::initialize");
    }
    fn post(&mut self, event: AoEvent) -> bool {
        println!("ActiveObjectInternal::post");
        let mut _guard: std::sync::MutexGuard<'_, bool>;

        match self.mutex.lock() {
            Ok(g) => {
                _guard = g;
            }
            Err(_) => todo!(),
        }

        let mut ret: bool = false;
        let queue_size: usize = self.queue.len();

        // If the queue is not full. Add the new event to it.
        // TODO 100 should be self.queue_size
        if queue_size < 100 {
            match self.queue.push_back(event) {
                Ok(_) => {
                    
                }
                Err(_) => todo!(),
            };

            // Unlock the queue mutex
            //drop(guard);

            if 0 == queue_size {
                // notify the conditional variable
                self.conditional_var.notify_one();
            }

            ret = true;
        }
        ret
    }
    fn post_urgent(&mut self, event: AoEvent) -> bool {
        println!("ActiveObjectInternal::post_urgent");
        // Lock the queue mutex.
        let mut _guard: std::sync::MutexGuard<'_, bool>;

        match self.mutex.lock() {
            Ok(g) => {
                _guard = g;
            }
            Err(_) => todo!(),
        }

        let mut ret: bool = false;
        let queue_size: usize = self.queue.len();

        // If the queue is not full. Add the new event to it.
        // todo 100 shuld be self.queue_size
        if queue_size < 100 {
            match self.queue.push_front(event) {
                Ok(_) => {

                }
                Err(_) => todo!(),
            };


            // unlock the queue mutex.
            //drop(guard);

            if 0 == queue_size {
                // notify the conditional variable.
                self.conditional_var.notify_one();
            }
            ret = true;
        }
        ret
    }
    fn publish(&mut self, event: AoEvent) -> bool {
        println!("ActiveObjectInternal::publish");
        true
    }
    fn publish_urgent(&mut self, event: AoEvent) -> bool {
        println!("ActiveObjectInternal::publish_urgent");
        true
    }
    fn process_one_event(&mut self) -> bool {
        println!("ActiveObjectInternal::process_one_event");
        let guard: std::sync::MutexGuard<'_, bool>;

        match self.mutex.lock() {
            Ok(g) => {
                guard = g;
            }
            Err(_) => todo!(),
        }

        let event: AoEvent;

        while self.queue.is_empty() {
            match self.conditional_var.wait(self.mutex.lock().unwrap()) {
                Ok(_) => {

                }
                Err(_) => todo!(),
            }
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
    fn exit_task(&mut self) {
        println!("ActiveObjectInternal::exit_task");
        self.exit = true;
    }
    fn task(&mut self) {
        println!("ActiveObjectInternal::task");
        while !self.exit {
            self.process_one_event();
        }
    }
}

#[derive(Clone)]
pub struct ActiveObject {
    // hold the internal active object
    internal: Arc<Mutex<ActiveObjectInternal>>
}

impl ActiveObject {
    pub fn new() -> ActiveObject {
        println!("ActiveObject::new");
        ActiveObject {
            internal : Arc::new(Mutex::new(ActiveObjectInternal {
                exit: false, 
                //thread_builder: thread::Builder::new().name(name).stack_size(stack_size), 
                conditional_var: Condvar::new(), 
                mutex: Mutex::new(false), 
                queue: ArrayDeque::new(), 
                queue_size: 100,
                stack_size: 100,
                state_machine: Arc::new(Mutex::new(InternalStateMachine::new()))
            }))
        }
    }

    pub fn initialize(&self, initial_state: StateT) {
        match self.internal.lock() {
            Ok(ism) => {
                match ism.state_machine.lock() {
                    Ok(mut sm) => {
                        sm.initialize(initial_state);
                    }
                    Err(_) => todo!()
                }
            }
            Err(_) => todo!()
        }
    }

    pub fn post(&mut self, event: AoEvent) -> bool {
        match self.internal.lock() {
            Ok(mut ism) => {
                ism.post(event)
            }
            Err(_) => todo!()
        }
    }

    pub fn start(&self) {
        println!("ActiveObject::start");
        let local_self: Arc<Mutex<ActiveObjectInternal>> = self.internal.clone();
        local_self.lock().unwrap().initialize();
        thread::spawn(move || {
            match local_self.lock() {
                Ok(mut ls) => {
                    ls.task();
                }
                Err(_) => todo!(),
            }
        });
    }   

    pub fn stop(&self) {
        println!("ActiveObject::stop");
        //let handle: thread::Thread = thread::current();
        match self.internal.lock() {
            Ok(mut i) => {
                i.exit_task();
                i.post_urgent(AoEvent {signal: AoSignal::AoProbeSig});
            }
            Err(_) => todo!(),
        }
        //handle.join().unwrap();
    } 
}