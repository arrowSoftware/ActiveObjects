mod state_machine;
mod active_object;
use state_machine::{StateMachine, AOEvent, AOSignal, State};
use active_object::{ActiveObject};

struct BootState {

}

struct IdleState {

}

impl State for BootState {
    fn new() -> BootState {
        BootState {  

        }
    }

    fn run(&self, event: AOEvent) {
        match event.sig {
            AOSignal::AoEnterSig => {
                println!("Enter Signal");
               // transition_to(IdleState);
            },
            AOSignal::AoExitSig => {
                println!("Exit Signal");
                //handled();
            },
            _ => {
                println!("Default signal handler");
               // handled();
            }
        }
    }
}

impl State for IdleState {
    fn new() -> IdleState {
        IdleState {  

        }
    }

    fn run(&self, event: AOEvent) {
        match event.sig {
            AOSignal::AoEnterSig => {
                println!("Enter Signal");
                //transition_to(IdleState);
            },
            AOSignal::AoExitSig => {
                println!("Exit Signal");
               // handled();
            },
            _ => {
                println!("Default signal handler");
                //handled();
            }
        }
    }
}

struct TestActiveObject {
    super_ao: ActiveObject,
    boolean_data: bool,
    int32_data: i32,
    boot_state: BootState,
    idle_state: IdleState
}

impl TestActiveObject {
    fn new() -> TestActiveObject {
        let mut me: TestActiveObject = TestActiveObject { 
            super_ao: ActiveObject::new(),
            boolean_data: false, 
            int32_data: 0, 
            boot_state: BootState::new(),
            idle_state: IdleState::new()
        };
        me.initialize();
        me
    }

    fn initialize(&self) {
        // Subscribe to signals...
        // Create timers...
        self.set_current_state(self.boot_state);
    }
}

fn main() {
    let mut test_active_object: TestActiveObject = TestActiveObject::new();
    test_active_object.initialize();
    test_active_object.start();
}
