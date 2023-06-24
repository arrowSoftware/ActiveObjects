use std::sync::{Arc, Mutex, Condvar};
use std::thread::{self, Builder};
use arraydeque::ArrayDeque;

use crate::state_machine::{StateMachine, AOEvent, AOSignal};

#[derive(Debug)]
struct ActiveObjectInner {
    exit: bool,
    //thread_builder: Builder,
    conditional_var: Condvar,
    mutex: Mutex<bool>,
    queue: ArrayDeque<AOEvent, 100>,
    queue_size: u32,
    stack_size: usize,
    state_machine: StateMachine
}

#[derive(Debug)]
pub struct ActiveObject {
    inner: Arc<Mutex<ActiveObjectInner>>
}

impl ActiveObject {
    pub fn new(name: String, queue_size: u32, stack_size: usize) -> Self {
        ActiveObject { 
            inner: Arc::new(Mutex::new(ActiveObjectInner {
                exit: false, 
                //thread_builder: thread::Builder::new().name(name).stack_size(stack_size), 
                conditional_var: Condvar::new(), 
                mutex: Mutex::new(false), 
                queue: ArrayDeque::new(), 
                queue_size: queue_size,
                stack_size: stack_size,
                state_machine: StateMachine::new()
            }))
        }
    }

    pub fn start(&mut self) {
        let local_self: Arc<Mutex<ActiveObjectInner>> = self.inner.clone();
        local_self.initialize();
        thread::spawn(move || {
            local_self.lock().unwrap().task();
        });
    }
}

impl ActiveObjectInner {
    fn initialize(self) {
        // Set the state to the initial psuedo state.
        // execute the enter event on the psuedo state.
        // Enter the real initial state with the enter event.
        !todo!()
    }

    fn post(&mut self, event: AOEvent) -> bool {
        // Lock the queue mutex.
        let guard: std::sync::MutexGuard<'_, bool> = self.mutex.lock().unwrap();
        let mut ret: bool = false;
        let queue_size: usize = self.queue.len();

        // If the queue is not full. Add the new event to it.
        // todo 100 shuld be self.queue_size
        if queue_size < 100 {
            self.queue.push_back(event);

            // unlock the queue mutex.
            drop(guard);

            if 0 == queue_size {
                // notify the conditional variable.
                self.conditional_var.notify_one();
            }
            ret = true;
        }
        ret
    }

    fn post_urgent(&mut self, event: AOEvent) -> bool {
        // Lock the queue mutex.
        let guard: std::sync::MutexGuard<'_, bool> = self.mutex.lock().unwrap();
        let mut ret: bool = false;
        let queue_size: usize = self.queue.len();

        // If the queue is not full. Add the new event to it.
        // todo 100 shuld be self.queue_size
        if queue_size < 100 {
            self.queue.push_front(event);

            // unlock the queue mutex.
            drop(guard);

            if 0 == queue_size {
                // notify the conditional variable.
                self.conditional_var.notify_one();
            }
            ret = true;
        }
        ret
    }

    fn publish(&mut self, event: AOEvent) -> bool {
        todo!()
    }

    fn publish_urgent(&mut self, event: AOEvent) -> bool {
        todo!()
    }

    fn stop(&mut self) {
        let handle: thread::Thread = thread::current();
        self.exit_task();
        self.post_urgent(AOEvent {sig:AOSignal::AoProbeSig});
        handle.join().unwrap();    
    }

    fn process_one_event(&mut self) -> bool {
        let guard: std::sync::MutexGuard<'_, bool> = self.mutex.lock().unwrap();
        let event: AOEvent;

        while self.queue.is_empty() {
            self.conditional_var.wait(guard);
        }

        event = self.queue.front();
        self.queue.pop_front();
        drop(guard);
        self.state_machine.processEvent(event)
    }

    fn exit_task(&mut self) {
        self.exit = true;
    }

    fn task(&mut self) {
        self.state_machine.initialize();

        while !self.exit {
            self.process_one_event();
        }
    }
}
