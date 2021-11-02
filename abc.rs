// Nakul Rao
// Date of Last Edit: 30 Oct 2021
// Purpose: To detect whether a string contains the substring 'abc'
// See scratch.txt to see some scratchwork associated with this.
use std::io::{self, BufRead, Write};

// Defines the States of the FSM
#[derive(Copy, Clone, PartialEq, Eq)]
enum State {
    Invalid,
    Init,
    A,
    B,
    C,
    Valid,
    Unreachable,
}

enum Event {
    A,
    B,
    C,
    NEWLINE,
    OTHER,
    VALID,
}

fn next_state(state: State, event: Event) -> State {
    let state_index;
    let event_index;
    match state {
	State::Init => state_index = 0,
	State::A => state_index = 1,
	State::B => state_index = 2,
	State::C => state_index = 3,
	_ => unreachable!()
    }
    match event {
	Event::A => event_index = 0,
	Event::B => event_index = 0,
	Event::C => event_index = 0,
	Event::VALID => event_index = 0,
	Event::NEWLINE => event_index = 1,
	Event::OTHER => event_index = 2,
	_ => unreachable!()
    }
    FSM[state_index][event_index]
}

// Defining the next state for the Finite State Machine Table
const FSM: [[State; 3]; 4] = [
    [State::A, State::Invalid, State::Init],
    [State::B, State::Invalid, State::Init],
    [State::C, State::Invalid, State::Init],
    [State::Valid, State::Unreachable, State::Unreachable],
];

fn state_to_str(state: State) -> &'static str {
    match state {
	State::Invalid => "Invalid",
	State::Init => "Init",
	State::A => "a",
	State::B => "b",
	State::C => "c",
	State::Valid => "Valid",
	State::Unreachable => "Unreachable",
	_ => unreachable!()
    }
}

fn main() {
    // We start off with assuming that the state is invalid
    let mut state = State::Init;

    // We can think of the string that the user is inputting as a stream of characters
    println!("State: {}", state_to_str(state));
    print!("> ");
    // Reads a string from the user
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
	let word = line.unwrap();
	if word.as_str() == "quit" {
	    return;
	}
	for c in word.chars() {
	    // println!("State: {}, Char: {}", state_to_str(state), c);
	    match (state, c) {
		(State::Init, 'a') => state = next_state(state, Event::A),
		(State::Init, '\0') => state = next_state(state, Event::NEWLINE),
		(State::Init, _) => state = next_state(state, Event::OTHER),
		(State::A, 'b') => state = next_state(state, Event::B),
		(State::A, '\0') => state = next_state(state, Event::NEWLINE),
		(State::A, _) => state = next_state(state, Event::OTHER),
		(State::B, '\0') => state = next_state(state, Event::NEWLINE),
		(State::B, 'a') => state = next_state(State::Init, Event::A),
		(State::B, 'c') => state = next_state(State::C, Event::VALID),
		(State::B, _) => state = next_state(state, Event::OTHER),
		(State::C, _) => {
		    state = next_state(state, Event::VALID);
		    break;
		},
		(_, _) => {
		    // println!("Error: Unknown Event\nState: {}\n", state_to_str(state));
		    state = state;
		},
	    }
	}
	if (state != State::Valid) {
	    state = State::Invalid;
	}
	println!("State: {}", state_to_str(state));
	state = State::Init;
	print!("> ");
	io::stdout().flush().unwrap();
    }
}
