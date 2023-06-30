use crate::active_object::ActiveObject;

// There can only be one controller. TODO singleton
// The controller will reference all created active objects

struct ActiveObjectController<'a> {
    active_objects: Vec<&'a ActiveObject>
}

impl ActiveObjectController<'_> {
    pub fn new() -> ActiveObjectController<'static> {
        ActiveObjectController {
            active_objects: Vec::new()
        }
    }
    pub fn register(&mut self, active_object: &mut ActiveObject) {
        //self.active_objects.push(active_object);
    }
    pub fn process_one_event(&self) {
        // If there is something on the publish queue
        // pop the item from the queue
        // send the item to all active objects
    }
    pub fn run(&self) {
        loop {
            self.process_one_event();
        }
    }
}
