use arraydeque::{ArrayDeque, CapacityError};

use crate::ao_event::AoEvent;

/**
 * AoComms is the communication interface between (the active object and other active
 * objects) && (The current state and itself).
 */
pub struct AoComms<'a> {
//    pub post_queue: ArrayDeque<AoEvent, 100>,
//    pub publish_queue: ArrayDeque<AoEvent, 100>,
    pub post_queue: &'a mut ArrayDeque<AoEvent, 100>,
//    pub publish_queue: &'a ArrayDeque<AoEvent, 100>
}

impl AoComms<'_> {
    /**
     * The new method creates the AoComms object containing the post/publish queues.
     * @return AoComms object
     */
    pub fn new(post_queue: &mut ArrayDeque<AoEvent, 100>) -> AoComms<'_> {
        AoComms {
            // The post queue, internal to the current state.
            post_queue: post_queue,
            // The publish queue, used to communication with other active objects.
//            publish_queue: publish_queue,
        }
    }

    /**
     * The post function will add an event to the end of the post queue, the post methods
     * are only internal to the current state.
     * @param self The AoComms instance
     * @param event The event to post to the state machine.
     * @return true if the post was successful, false otherwise.
     */
    pub fn post(&mut self, event: AoEvent) -> bool {
        println!("AoComms::post {:?}", event);
        let mut success: bool = false;

        // If the queue is not full. Add the new event to it.
        if !self.post_queue.is_full() {
            match self.post_queue.push_back(event) {
                Ok(_) => {
                    success = true;
                }
                Err(CapacityError { element: _ }) => {
                    // Capicity error will only occur if the queue is full when a push_back occurs.
                    // TODO this might be impossble code to reach becuase of the is_full check wrapping this
                    // match.
                    success = false;
                },
            };
        }
        success
    }

    /**
     * The postUrgent function will add an event to the front of the post queue, the post methods
     * are only internal to the current state.
     * @param self The AoComms instance
     * @param event The event to post to the state machine.
     * @return true if the post was successful, false otherwise.
     */
    pub fn postUrgent(&mut self, event: AoEvent) -> bool {
        println!("AoComms::postUrgent {:?}", event);
        let mut success: bool = false;

        // If the queue is not full. Add the new event to it.
        if !self.post_queue.is_full() {
            match self.post_queue.push_front(event) {
                Ok(_) => {
                    success = true;
                }
                Err(CapacityError { element: _ }) => {
                    // Capicity error will only occur if the queue is full when a push_back occurs.
                    // TODO this might be impossble code to reach becuase of the is_full check wrapping this
                    // match.
                    success = false;
                },
            };
        }
        success
    }

    /**
     * The publish function will add an event to the back of the publish queue, the publish methods
     * send events to aother active objects.
     * @param self The AoComms instance
     * @param event The event to publish to the state machine.
     * @return true if the publish was successful, false otherwise.
     */
    pub fn publish(&mut self, event: AoEvent) -> bool {
        println!("AoComms::postUrgent {:?}", event);
        todo!();
        /*
        let mut success: bool = false;
        let (lock, cvar) = &self.publish_queue;
        let mut queue = match lock.lock() {
            Ok(q) => {
                q
            },
            Err(_) => { todo!() }
        };

        // If the queue is not full. Add the new event to it.
        if !queue.is_full() {
            match queue.push_back(event) {
                Ok(_) => {
                    success = true;
                }
                Err(CapacityError { element: _ }) => {
                    // Capicity error will only occur if the queue is full when a push_back occurs.
                    // TODO this might be impossble code to reach becuase of the is_full check wrapping this
                    // match.
                    success = false;
                },
            };

            if queue.is_empty() {
                cvar.notify_one();
            }
        }
        success
        */
    }

    /**
     * The publish function will add an event to the front of the publish queue, the publish methods
     * send events to aother active objects.
     * @param self The AoComms instance
     * @param event The event to publish to the state machine.
     * @return true if the publish was successful, false otherwise.
     */
    pub fn publishUrgent(&mut self, event: AoEvent) -> bool {
        println!("AoComms::publishUrgent {:?}", event);
        todo!();
        /*
        let mut success: bool = false;
        let (lock, cvar) = &self.publish_queue;
        let mut queue = match lock.lock() {
            Ok(q) => {
                q
            },
            Err(_) => { todo!() }
        };

        // If the queue is not full. Add the new event to it.
        if !queue.is_full() {
            match queue.push_front(event) {
                Ok(_) => {
                    success = true;
                }
                Err(CapacityError { element: _ }) => {
                    // Capicity error will only occur if the queue is full when a push_back occurs.
                    // TODO this might be impossble code to reach becuase of the is_full check wrapping this
                    // match.
                    success = false;
                },
            };

            if queue.is_empty() {
                cvar.notify_one();
            }
        }
        success
        */
    }
}
