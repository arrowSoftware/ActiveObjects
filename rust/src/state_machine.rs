use arraydeque::ArrayDeque;

use crate::ao_signal::AoSignal;
use crate::ao_event::AoEvent;
use crate::action::Action;
use crate::state::{PsuedoState, StateT};
use crate::ao_comms::AoComms;

pub struct StateMachine {
    // The current state of the state machine.
    pub current_state: StateT,
    // The queue for posting event to states. Internal to the state machine.
    post_queue: ArrayDeque<AoEvent, 100>
}

impl StateMachine {
    /**
     * Creates a new StateMachine object.
     * @return StateMachine object
     */
    pub fn new() -> StateMachine {
        StateMachine {
            current_state: Box::new(PsuedoState::new()),
            post_queue: ArrayDeque::new()
        }
    }
}

// State machine used to process events for states and handle state transitions.
impl StateMachine {
    /**
     * Sets the initial state and executed the enter
     * event on that state.
     * @param self StateMachine instance.
     * @param initial_state The initial state of the state machine
     */
    pub fn initialize(&mut self, initial_state: StateT) {
        println!("StateMachine::initialize");
        // Set the state to the initial state.
        self.current_state = initial_state;

        // Execute the enter event on the new initial state.
        self.process_event(AoEvent::new(AoSignal::AoEnterSig));
    }

    /**
     * Will pop the next event off the front of the post queue if there is
     * an event on the queue, returning it to the caller.
     * @param self StateMachine instance
     * @return Option None if there is no event on the queue,
     * Some(AoEvent) if there is
     */
    pub fn get_next_event(&mut self) -> Option<AoEvent> {
        let opt : Option<AoEvent>;

        if self.post_queue.is_empty() {
            opt = None
        } else {
            match self.post_queue.front() {
                Some(e) => {
                    opt = Some(*e);
                }
                None => {
                    opt = None;
                }
            }
            self.post_queue.pop_front();
        }
        opt
    }

    /**
     * Will execute the exit event on the current state, switch to the new
     * target state then execute the enter event on the new state.
     * @param self StateMachine instance
     * @param target_state State to transition to.
]     */
    pub fn transition_to(&mut self, target_state: StateT) {
        println!("StateMachine::transition_to");
        // Execute the exit event on the current state
        self.current_state.run(AoEvent::new(AoSignal::AoExitSig), &mut AoComms::new(&mut self.post_queue));
        // Update the current state with the new target state.
        self.current_state = target_state;
        // Execute the enter event on the new current event.
        self.current_state.run(AoEvent::new(AoSignal::AoEnterSig), &mut AoComms::new(&mut self.post_queue));
    }

    /**
     * Does nothing, just psuedo code for logging atm.
     * @param self StateMachine instance
     */
    pub fn handled(&mut self) {
        println!("StateMachine::handled");
    }

    /**
     * Will execute an incoming event on the current state and transition to the
     * new state if the return from state indicates, otherwise it will remain
     * in the current state.
     * @param self StateMachine instance
     * @param event AoEvent to process.
     */
    pub fn process_event(&mut self, event: AoEvent) {
        println!("StateMachine::process_event {:?}", event);

        // Call current state with event
        match self.current_state.run(event, &mut AoComms::new(&mut self.post_queue)) {
            Action::Handled => {
                println!("StateMachine::process_event::Handled");
                self.handled();
            },
            Action::TransitionTo(target_state) => {
                println!("StateMachine::process_event::TransitionTo");
                self.transition_to(target_state);
            }
        }
    }

    pub fn tick(&mut self) {
        let expired_timers: Vec<AoSignal>;
        let mut ao_comms: AoComms = AoComms::new(&mut self.post_queue);
        // Call the timer event on the states to execute the tick function.
        expired_timers = self.current_state.get_super().tick();
        // Post all the expired timers signals.
        for signal in expired_timers.iter() {
            ao_comms.post(AoEvent::new(*signal));
        }
    }
}
