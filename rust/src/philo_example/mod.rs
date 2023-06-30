use crate::action::Action;
use crate::action::Action::*;
use crate::state::{State, AoSuper};
use crate::ao_event::AoEvent;
use crate::ao_signal::AoSignal::*;
use crate::active_object::ActiveObject;
use crate::ao_comms::AoComms;
use crate::ao_timer::AoTimer;

struct PhiloInitial {
    pub ao_super: AoSuper
}
impl PhiloInitial {
    fn new() -> PhiloInitial {
        println!("PhiloInitial::new");
        let mut state = PhiloInitial {
            ao_super: AoSuper::new()
        };
        AoTimer::new(&mut state.ao_super, AoStdTimeoutSig, 100);
        state
    }
}

struct PhiloThinking {
    pub ao_super: AoSuper
}
impl PhiloThinking {
    fn new() -> PhiloThinking {
        println!("PhiloThinking::new");
        PhiloThinking {
            ao_super: AoSuper::new()
        }
    }
}

struct PhiloHungry {
    pub ao_super: AoSuper
}
impl PhiloHungry {
    fn new() -> PhiloHungry {
        println!("PhiloHungry::new");
        PhiloHungry {
            ao_super: AoSuper::new()
        }
    }
}

struct PhiloEating {
    pub ao_super: AoSuper
}
impl PhiloEating {
    fn new() -> PhiloEating {
        println!("PhiloEating::new");
        PhiloEating {
            ao_super: AoSuper::new()
        }
    }
}

impl State for PhiloInitial {
    fn run(&mut self, event: AoEvent, ao_comms: &mut AoComms) -> Action {
        println!("PhiloInitial::run {:?}", event);
        let ret: Action;
        match event.signal {
            AoEnterSig => {
                println!("PhiloInitial::run::Enter event");
                ret = TransitionTo(Box::new(PhiloThinking::new()));
            }
            AoExitSig => {
                println!("PhiloInitial::run::Exit event");
                ret = Handled;
            }
            _ => {
                println!("PhiloInitial::rune::Default signal handler");
                ret = Handled;
            }
        }
        ret
    }
    fn get_super(&mut self) -> AoSuper {
        self.ao_super.clone()
    }
}

impl State for PhiloThinking {
    fn run(&mut self, event: AoEvent, ao_comms: &mut AoComms) -> Action {
        println!("PhiloThinking::run {:?}", event);
        let ret: Action;
        match event.signal {
            AoEnterSig => {
                // TODO start timer for AoStdTimeoutSig
                //let timer: Timer = Timer::new(AoStdTimeoutSig, 100);
                //timer.arm();
                println!("PhiloThinking::run::Enter event");
                ret = Handled;
            }
            AoStdTimeoutSig => {
                println!("PhiloThinking::run::AoStdTimeoutSig event");
                ret = TransitionTo(Box::new(PhiloHungry::new()));
            }
            AoEatSig | AoDoneSig => {
                ret = Handled;
            }
            AoExitSig => {
                println!("PhiloThinking::run::Exit event");
                ret = Handled;
            }
            _ => {
                println!("PhiloThinking::rune::Default signal handler");
                ret = Handled;
            }
        }
        ret
    }
    fn get_super(&mut self) -> AoSuper {
        self.ao_super.clone()
    }
}

impl State for PhiloHungry {
    fn run(&mut self, event: AoEvent, ao_comms: &mut AoComms) -> Action {
        println!("PhiloHungry::run {:?}", event);
        let ret: Action;
        match event.signal {
            AoEnterSig => {
                println!("PhiloHungry::run::Enter event");
                ao_comms.post(AoEvent::new(AoHungrySig));
                ret = Handled;
            }
            AoEatSig => {
                println!("PhiloHungry::run::AoEatSig event");
                ret = TransitionTo(Box::new(PhiloEating::new()));
            }
            AoDoneSig => {
                println!("PhiloHungry::run::AoDoneSig event");
                ret = Handled;
            }
            AoExitSig => {
                println!("PhiloHungry::run::Exit event");
                ret = Handled;
            }
            _ => {
                println!("PhiloHungry::rune::Default signal handler");
                ret = Handled;
            }
        }
        ret
    }
    fn get_super(&mut self) -> AoSuper {
        self.ao_super.clone()
    }
}

impl State for PhiloEating {
    fn run(&mut self, event: AoEvent, ao_comms: &mut AoComms) -> Action {
        println!("PhiloEating::run {:?}", event);
        let ret: Action;
        match event.signal {
            AoEnterSig => {
                // TODO start timer for aoStdTimeoutSig
                println!("PhiloEating::run::Enter event");
                ret = Handled;
            }
            AoStdTimeoutSig => {
                println!("PhiloEating::run::AoStdTimeoutSig event");
                ret = TransitionTo(Box::new(PhiloThinking::new()));
            }
            AoEatSig | AoDoneSig => {
                println!("PhiloEating::run::AoEatSig | AoDoneSig event");
                ret = Handled;
            }
            AoExitSig => {
                println!("PhiloEating::run::Exit event");
                ao_comms.post(AoEvent::new(AoDoneSig));
                ret = Handled;
            }
            _ => {
                println!("PhiloEating::rune::Default signal handler");
                ret = Handled;
            }
        }
        ret
    }
    fn get_super(&mut self) -> AoSuper {
        self.ao_super.clone()
    }
}

pub fn run() {
    let active_object : ActiveObject = ActiveObject::new();
    let initial_state = Box::new(PhiloInitial::new());
    active_object.start(initial_state);

    loop {}
}