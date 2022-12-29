use std::collections::HashMap;

use crate::term::Term;

#[derive(PartialEq, Debug)]
pub struct Clause {
    pub head: Term,
    pub body: Vec<Term>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Predicate {
    pub name: String,
    arity: usize,
}
impl Predicate {
    pub fn from_clause(clause: &Clause) -> Option<Predicate> {
	Self::from_term(&clause.head)
    }

    pub fn from_term(term: &Term) -> Option<Predicate> {
        match term.clone() {
	    Term::Str(f, args) => Some(Predicate {
		name: f.clone(),
		arity: args.len(),
	    }),
	    Term::Atom(f) => Some(Predicate {
		name: f.clone(),
		arity: 0,
	    }),
	    _ => None
	}
    }
}

pub struct Database {
    data: HashMap<Predicate, Vec<Clause>>
}

impl Database {
    pub fn new() -> Self {
	Database {
	    data: HashMap::new()
	}
    }

    pub fn add_clause(&mut self, clause: Clause) {
	let head = clause.head.clone();
	if let Some(predicate_key) = Predicate::from_clause(&clause) {
            {

		self.data.entry(predicate_key.clone()).or_insert(Vec::new());
	    }
	    {
		let predicate = self.data.get_mut(&predicate_key).unwrap();
		predicate.push(clause);
	    }
	}
    }

    pub fn get_clauses(&self, predicate: &Predicate) -> Option<&Vec<Clause>> {
	self.data.get(predicate)
    }

    pub fn clear_all(&mut self) {
	self.data = HashMap::new();
    }

    pub fn clear_predicate(&mut self, predicate: &Predicate) {
	self.data.remove(predicate);
    }
}

#[test]
fn add_rule() {
    let clause = Clause {
	head: Term::Str("mortal".into(), vec![Term::Var("X".into())]),
	body: vec![Term::Str("human".into(), vec![Term::Var("X".into())])],
    };

    let mut db = Database::new();
    db.add_clause(clause);
    let clauses = db.get_clauses(&Predicate { name: "mortal".into(), arity: 1}).unwrap();
    assert_eq!(clauses.len(), 1);
}

#[test]
fn add_fact() {
    let clause = Clause {
	head: Term::Str("human".into(), vec![Term::Atom("socrates".into())]),
	body: vec![Term::Atom("true".into())],
    };

    let mut db = Database::new();
    db.add_clause(clause);
    let clauses = db.get_clauses(&Predicate { name: "human".into(), arity: 1}).unwrap();
    assert_eq!(clauses.len(), 1);
    assert_eq!(clauses[0].body, vec![Term::Atom("true".into())]);
    let clauses = db.get_clauses(&Predicate { name: "human".into(), arity: 0});
    assert!(clauses.is_none());
}
