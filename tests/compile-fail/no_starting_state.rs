use smlang::statemachine;

statemachine! { //~ ERROR No starting state defined, indicate the starting state with a *
    State1 + Event1 = State2,
    State2 + Event2 = State3,
}

fn main() {}