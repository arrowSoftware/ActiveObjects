use std::sync::{Arc, Mutex, Condvar};
use std::thread::{self, Builder};
use arraydeque::ArrayDeque;

use crate::AoSignal::AoSignal;
use crate::AoEvent::AoEvent;
use crate::State::{State, StateT};

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
        let guard: std::sync::MutexGuard<'_, bool> = self.mutex.lock().unwrap();
        let mut ret: bool = false;
        let queue_size: usize = self.queue.len();

        // If the queue is not full. Add the new event to it.
        // TODO 100 should be self.queue_size
        if queue_size < 100 {
            self.queue.push_back(event);

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
        let guard: std::sync::MutexGuard<'_, bool> = self.mutex.lock().unwrap();
        let mut ret: bool = false;
        let queue_size: usize = self.queue.len();

        // If the queue is not full. Add the new event to it.
        // todo 100 shuld be self.queue_size
        if queue_size < 100 {
            self.queue.push_front(event);

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
        let guard: std::sync::MutexGuard<'_, bool> = self.mutex.lock().unwrap(); // TODO
        let event: AoEvent;

        while self.queue.is_empty() {
            self.conditional_var.wait(self.mutex.lock().unwrap());
        }

        event = *self.queue.front().unwrap();
        self.queue.pop_front();
        //drop(guard);
        self.state_machine.lock().unwrap().process_event(event);
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
        self.internal.lock().unwrap().state_machine.lock().unwrap().initialize(initial_state);
    }

    pub fn start(&self) {
        println!("ActiveObject::start");
        let local_self: Arc<Mutex<ActiveObjectInternal>> = self.internal.clone();
        local_self.lock().unwrap().initialize();
        thread::spawn(move || {
            local_self.lock().unwrap().task();
        });
    }   

    pub fn stop(&self) {
        println!("ActiveObject::stop");
        let handle: thread::Thread = thread::current();
        self.internal.lock().unwrap().exit_task();
        self.internal.lock().unwrap().post_urgent(AoEvent {signal: AoSignal::AoProbeSig});
//        handle.join().unwrap();
    } 
}