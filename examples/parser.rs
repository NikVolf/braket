extern crate brakets;

#[derive(Debug)]
enum Symbol { One, Zero, Up, Down, Inward, Outward, Left, Right, Plus, Minus}

impl From<char> for Symbol {
    fn from(c: char) -> Symbol {
        match c {
            'u' => Symbol::Up,
            'd' => Symbol::Down,
            '1' => Symbol::One,
            '0' => Symbol::Zero,
            'i' => Symbol::Inward,
            'o' => Symbol::Outward,
            'l' => Symbol::Left,
            'r' => Symbol::Right,
            '+' => Symbol::Plus,
            '-' => Symbol::Minus,
            _ => panic!("Caller should not pass other than ud10iolr+-"),
        }
    }
}

#[derive(Debug)]
enum Item {
    Bra(Symbol),
    Ket(Symbol),
}

#[derive(Debug)]
enum State {
    Initial,
    StartedBra,
    StartedKet,
    IdentifiedBra(Symbol),
    IdentifiedKet(Symbol),
    FinishedBra,
    FinishedKet,
}

#[derive(Debug)]
enum Error {
    ExpectedBraOrKet,
    ExpectedIdent,
    UnexpectedFinalizer,
    UnexpectedSymbol,
    Unexpected,
}

#[derive(Debug)]
struct ErrorState {
    kind: Error,
    index: usize,
    encounter: char,
}

fn main() {
    let expression = ::std::env::args().nth(1).expect("should be 1 argument");

    let mut stack = Vec::new();

    let mut state = State::Initial;

    let mut index: usize = 0;

    let mut error_state: Option<ErrorState> = None;

    for chr in expression.chars() {
        index += 1;
        match chr {
            '<' => {
                match state {
                    State::Initial | State::FinishedBra | State::FinishedKet => {
                        state = State::StartedBra;
                    },
                    _ => {
                        error_state = Some(ErrorState {
                            index: index,
                            encounter: chr,
                            kind: Error::Unexpected,
                        });
                    }
                }
            },
            '|' => {
                match state {
                    State::Initial => {
                        state = State::StartedKet;
                    },
                    State::IdentifiedBra(s) => {
                        stack.push(Item::Bra(s));
                        state = State::FinishedBra;
                    },
                    _ => {
                        error_state = Some(ErrorState {
                            index,
                            encounter: chr,
                            kind: Error::Unexpected,
                        });
                    }
                }
            },
            '>' => {
                match state {
                    State::IdentifiedKet(s) => {
                        stack.push(Item::Ket(s));
                        state = State::FinishedKet;
                    },
                    _ => {
                        error_state = Some(ErrorState {
                            index,
                            encounter: chr,
                            kind: Error::UnexpectedFinalizer,
                        });
                    }
                }
            },
            'u' | 'd' => {
                match state {
                    State::StartedBra => {
                        state = State::IdentifiedBra(Symbol::from(chr));
                    },
                    State::StartedKet | State::FinishedBra => {
                        state = State::IdentifiedKet(Symbol::from(chr));
                    },
                    _ => {
                        error_state = Some(ErrorState {
                            index,
                            encounter: chr,
                            kind: Error::UnexpectedSymbol,
                        });
                    }
                }
            },
            _ => {
                error_state = Some(ErrorState {
                    index,
                    encounter: chr,
                    kind: Error::UnexpectedSymbol,
                });
            }
        }

        if let Some(_) = error_state {
            break;
        }
    }

    match error_state {
        Some(error_state) => {
            println!("ERROR: {:?}", error_state);
        },
        None => {
            println!("{:?}", stack);
        }
    };
}
