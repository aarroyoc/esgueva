use std::fmt;

#[derive(Debug, Clone)]
pub enum Term {
    Atom(String),
    Var(String),
    Str(String, Vec<Term>),
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
	match (self, other) {
	    (Term::Atom(x), Term::Atom(y)) => x == y,
	    (Term::Var(x), Term::Var(y)) => x == y,
	    _ => false
	}
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	match self {
	    Term::Atom(x) => write!(f, "{}", x),
	    Term::Var(x) => write!(f, "{}", x),
	    Term::Str(functor, args) => write!(f, "{}({})", functor, args.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(","))
	}
    }
}
