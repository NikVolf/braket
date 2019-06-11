extern crate brakets;

#[derive(Debug)]
enum Symbol { One, Zero, Up, Down, Inward, Outward, Left, Right, Plus, Minus}

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
                        match chr {
                            'u' => state = State::IdentifiedBra(Symbol::Up),
                            'd' => state = State::IdentifiedBra(Symbol::Down),
                            _ => {
                                error_state = Some(ErrorState {
                                    index,
                                    encounter: chr,
                                    kind: Error::Unexpected,
                                });
                            }
                        }
                    },
                    State::StartedKet | State::FinishedBra => {
                        match chr {
                            'u' => state = State::IdentifiedKet(Symbol::Up),
                            'd' => state = State::IdentifiedKet(Symbol::Down),
                            _ => {
                                error_state = Some(ErrorState {
                                    index,
                                    encounter: chr,
                                    kind: Error::Unexpected,
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
