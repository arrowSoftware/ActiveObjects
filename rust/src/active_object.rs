use std::sync::{Arc, Mutex, Condvar};
use std::thread::{self, JoinHandle};
use arraydeque::ArrayDeque;

use crate::state_machine::{StateMachine, AOEvent, AOSignal};

#[derive(Copy, Clone, Debug)]
struct ActiveObject {
    exit: bool,
    thread: JoinHandle<()>,
    conditional_var: Condvar,
    mutex: Mutex<i32>,
    // https://docs.rs/arraydeque/latest/arraydeque/struct.ArrayDeque.html#method.len
    queue: ArrayDeque<usize, 100>,
    queue_depth: u32
}

trait ActiveObjectController {
    fn new(queue_depth: u32) -> Self;
    // Create the thread and start the task method
    fn start(self);
    // push an event onto the active object queue
    fn post(&self, event: AOEvent) -> bool; 
    // push an event to the front of the active object queue
    fn post_urgent(&self, event: AOEvent) -> bool;
    // push an event to all active objects queue.
    fn publish(&self, event: AOEvent) -> bool;
    // push an event to the front of all active objects queue.
    fn publish_urgent(&self, event: AOEvent) -> bool;
    // Stop the thread.
    fn stop(&self);
    // Pull one event of the queue and process it.
    fn process_one_event(&self) -> bool;
    // Leave the task loop.
    fn exit_task(&self);
    // Loop inifinitely waiting for events in the queue to process.
    fn task(&self);
}

impl ActiveObjectController for ActiveObject {
    fn new(queue_depth: u32) -> Self {
        Self.mutex = Arc::new(Mutex::new(0));
    }

    fn start(mut self) {
        self.thread = thread::spawn(move || {
            self.task();
        });
    }

    fn post(&self, event: AOEvent) -> bool {
        // Lock the queue mutex.
        let mut guard: std::sync::MutexGuard<'_, i32> = self.mutex.lock().unwrap();
        let mut ret: bool = false;
        let queue_size: usize = self.queue.len();

        // If the queue is not full. Add the new event to it.
        if queue_size < 100 {
            self.queue.push_back(event);

            // unlock the queue mutex.
            Mutex::unlock(guard);

            if 0 == queue_size {
                // notify the conditional variable.
                self.conditional_var.notify_one();
            }
            ret = true;
        }
        ret
    }

    fn post_urgent(&self, event: AOEvent) -> bool {
        // Lock the queue mutex.
        let mut guard: std::sync::MutexGuard<'_, i32> = self.mutex.lock().unwrap();
        let mut ret: bool = false;
        let queue_size: usize = self.queue.len();

        // If the queue is not full. Add the new event to it.
        if queue_size < 100 {
            self.queue.push_front(event);

            // unlock the queue mutex.
            Mutex::unlock(guard);

            if 0 == queue_size {
                // notify the conditional variable.
                self.conditional_var.notify_one();
            }
            ret = true;
        }
        ret
    }

    fn publish(&self, event: AOEvent) -> bool {
        todo!()
    }

    fn publish_urgent(&self, event: AOEvent) -> bool {
        todo!()
    }

    fn stop(&self) {
        if thread {
            self.exit_task();
            self.post_urgent(AOEvent {sig=AOSignal::AoProbeSig});
            self.thread.join();
        }
    }

    fn process_one_event(&self) -> bool {
        todo!()
    }

    fn exit_task(&self) {
        todo!()
    }

    fn task(&self) {
        todo!()
    }
}
