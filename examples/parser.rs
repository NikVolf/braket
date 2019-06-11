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

impl Symbol {
    fn to_bra(&self) -> brakets::Bra2 {
        match &self {
            Symbol::Up | Symbol::One => brakets::Bra2::up(),
            Symbol::Down | Symbol::Zero => brakets::Bra2::down(),
            Symbol::Left | Symbol::Minus => brakets::Bra2::left(),
            Symbol::Right | Symbol::Plus => brakets::Bra2::right(),
            Symbol::Inward => brakets::Bra2::inw(),
            Symbol::Outward => brakets::Bra2::out(),
        }
    }

    fn to_ket(&self) -> brakets::Ket2 {
        match &self {
            Symbol::Up | Symbol::One => brakets::Ket2::up(),
            Symbol::Down | Symbol::Zero => brakets::Ket2::down(),
            Symbol::Left | Symbol::Minus => brakets::Ket2::left(),
            Symbol::Right | Symbol::Plus => brakets::Ket2::right(),
            Symbol::Inward => brakets::Ket2::inw(),
            Symbol::Outward => brakets::Ket2::out(),
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

#[derive(Debug)]
enum EvaluationState {
    None,
    Bra(brakets::Bra2),
    Ket(brakets::Ket2),
    Outer(brakets::Outer2),
    Scalar(brakets::Complex),
    Invalid,
}

impl ::std::fmt::Display for EvaluationState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            EvaluationState::None => write!(f, "None"),
            EvaluationState::Bra(ref bra) => write!(f, "<{}|", bra),
            EvaluationState::Ket(ref ket) => write!(f, "|{}>", ket),
            EvaluationState::Outer(ref outer) => write!(f, "{}", outer),
            EvaluationState::Scalar(ref scalar) => write!(f, "{}", scalar),
            EvaluationState::Invalid => write!(f, "Invalid"),
        }
    }
}

impl EvaluationState {
    fn eval(self, item: Item) -> Self {
        let result = match (self, item) {
            (EvaluationState::None, Item::Ket(ref s)) => EvaluationState::Ket(s.to_ket()),
            (EvaluationState::None, Item::Bra(ref s)) => EvaluationState::Bra(s.to_bra()),
            (EvaluationState::Bra(ref mut bra), Item::Ket(ref s)) => {
                EvaluationState::Scalar(bra.clone() * s.to_ket())
            },
            (EvaluationState::Ket(ref mut ket), Item::Bra(ref s)) => {
                EvaluationState::Outer(ket.clone() * s.to_bra())
            },
            (EvaluationState::Scalar(ref mut scalar), Item::Bra(ref s)) => {
                EvaluationState::Bra(s.to_bra() * *scalar)
            },
            (EvaluationState::Scalar(ref mut scalar), Item::Ket(ref s)) => {
                EvaluationState::Ket(s.to_ket() * *scalar)
            },
            (EvaluationState::Outer(ref mut outer), Item::Ket(ref s)) => {
                EvaluationState::Ket(outer.clone() * s.to_ket())
            },
            (EvaluationState::Outer(ref mut outer), Item::Bra(ref s)) => {
                EvaluationState::Bra(s.to_bra() * outer.clone())
            },
            _ => EvaluationState::Invalid,
        };

        result
    }
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
                    State::Initial | State::FinishedKet => {
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
            'u' | 'd' | 'o' | 'i' | '1' | '0' | '-' | '+' | 'l' | 'r' => {
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

            let mut evaluation_state = EvaluationState::None;

            for s in stack.drain(..) {
                evaluation_state = evaluation_state.eval(s);
            }

            println!("{}", evaluation_state);

        }
    };
}
