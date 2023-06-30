mod active_object_controller;
mod action;
mod active_object;
mod ao_event;
mod ao_signal;
mod state_machine;
mod state;
mod ao_comms;
mod ao_timer;

//mod example_app;
mod philo_example;

use crate::active_object::ActiveObject;
use crate::active_object_controller::ActiveObjectController;

fn main() {
    let mut ao_controller: ActiveObjectController = ActiveObjectController::new();

    let mut active_object : ActiveObject = ActiveObject::new();
    ao_controller.register(&mut active_object);

    active_object.start(Box::new(philo_example::PhiloInitial::new()));

    ao_controller.run();
}

