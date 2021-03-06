//! Looping state machine
//!
//! An example of using guards and actions.
//! A picture depicting the state machine can be found in the README.

use smlang::statemachine;

statemachine! {
    *State1 + Event1 [guard] / action1 = State2,
    State2 + Event2 [guard_fail] / action2 = State3,
}

#[derive(Debug)]
pub struct Context;

impl StateMachineContext for Context {
    fn guard(&self, _event: &Events) -> bool {
        // Always ok
        true
    }

    fn guard_fail(&self, _event: &Events) -> bool {
        // Always fail
        false
    }

    fn action1(&mut self, _event: &Events) {
        //println!("Action 1");
    }

    fn action2(&mut self, _event: &Events) {
        //println!("Action 1");
    }
}

fn main() {
    let mut sm = StateMachine::<Context>::new(Context);
    assert_eq!(sm.state(), States::State1);

    println!("Before action 1");

    // Go through the first guard and action
    let r = sm.process_event(Events::Event1);
    assert_eq!(r, Ok(States::State2));

    println!("After action 1");

    println!("Before action 2");

    // The action will never run as the guard will fail
    let r = sm.process_event(Events::Event2);
    assert_eq!(r, Err(Error::GuardFailed));

    println!("After action 2");

    // Now we are stuck due to the guard never returning true
    assert_eq!(sm.state(), States::State2);
}
