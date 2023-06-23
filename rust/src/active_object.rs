use std::sync::{Mutex, Condvar};
use std::thread::{self, Builder};
use arraydeque::ArrayDeque;

use crate::state_machine::{StateMachine, AOEvent, AOSignal};

#[derive(Debug)]
struct ActiveObject {
    exit: bool,
    thread_builder: Builder,
    conditional_var: Condvar,
    mutex: Mutex<bool>,
    // https://docs.rs/arraydeque/latest/arraydeque/struct.ArrayDeque.html#method.len
    queue: ArrayDeque<AOEvent, 100>,
    queue_size: u32,
    stack_size: usize
}

trait ActiveObjectController {
    fn new(name: String, queue_size: u32, stack_size: usize) -> Self;
    // Create the thread and start the task method
    fn start(self);
    // push an event onto the active object queue
    fn post(&mut self, event: AOEvent) -> bool; 
    // push an event to the front of the active object queue
    fn post_urgent(&mut self, event: AOEvent) -> bool;
    // push an event to all active objects queue.
    fn publish(&mut self, event: AOEvent) -> bool;
    // push an event to the front of all active objects queue.
    fn publish_urgent(&mut self, event: AOEvent) -> bool;
    // Stop the thread.
    fn stop(&mut self);
    // Pull one event of the queue and process it.
    fn process_one_event(&mut self) -> bool;
    // Leave the task loop.
    fn exit_task(&mut self);
    // Loop inifinitely waiting for events in the queue to process.
    fn task(&mut self);
}

impl ActiveObjectController for ActiveObject {
    fn new(name: String, queue_size: u32, stack_size: usize) -> Self {
        ActiveObject { 
            exit: false, 
            thread_builder: thread::Builder::new().name(name).stack_size(stack_size), 
            conditional_var: Condvar::new(), 
            mutex: Mutex::new(false), 
            queue: ArrayDeque::new(), 
            queue_size: queue_size,
            stack_size: stack_size
        }
    }

    fn start(mut self) {
        self.thread_builder.spawn(|| {
            self.task();
        }).unwrap();
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
        self.exit_task();
        self.post_urgent(AOEvent {sig:AOSignal::AoProbeSig});
        //self.thread_builder.join().unwrap();    
    }

    fn process_one_event(&mut self) -> bool {
        todo!()
    }

    fn exit_task(&mut self) {
        todo!()
    }

    fn task(&mut self) {
        todo!()
    }
}
