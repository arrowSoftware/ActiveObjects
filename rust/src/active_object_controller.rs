use crate::active_object::ActiveObject;

// There can only be one controller. TODO singleton
// The controller will reference all created active objects

pub struct ActiveObjectController {
    // A list of all active objects. Or a list of all active object join handles.
    // A publish queue called by the active objects to send messages between.
    active_objects: Vec<*mut ActiveObject>
}

impl ActiveObjectController {
    pub fn new() -> ActiveObjectController {
        ActiveObjectController {
            active_objects: Vec::new()
        }
    }
    pub fn register(&mut self, active_object: *mut ActiveObject) {
        self.active_objects.push(active_object);
    }
    pub fn process_one_event(&mut self) {
        let mut publish_events: Vec<AoEvent> = Vec::new();

        // pull an event from each active objects publish queue.
        for ao in self.active_objects.iter_mut() {
            match ao.get_next_event() {
                Some(e) => {
                    publish_events.push(e);
                }
                None => {}
            }
        }

        // send each publish event to all active objects.
        for event in publish_events.iter() {
            for ao in self.active_objects.iter_mut() {
                ao.store_publish_event(event);
            }
        }
    }
    pub fn run(&mut self) {
        loop {
            self.process_one_event();
        }
    }
}
