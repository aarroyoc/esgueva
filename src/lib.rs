use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Term {
    Number(i64),
    Atom(String),
    Str(String, Vec<Term>),
    Var(String),
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Substitution {
    name: String,
    value: Term,
}

pub fn unify(left: Term, right: Term) -> Option<HashSet<Substitution>> {
    match (left, right) {
	(Term::Number(x), Term::Number(y)) => {
	    if x == y {
		Some(HashSet::new())
	    } else {
		None
	    }
	},
	(Term::Atom(x), Term::Atom(y)) => {
	    if x == y {
		Some(HashSet::new())
	    } else {
		None
	    }
	}
	(Term::Var(name), value) => {
	    Some(HashSet::from([Substitution { name, value }]))
	}
	(value, Term::Var(name)) => {
	    Some(HashSet::from([Substitution { name, value}]))
	}
	(Term::Str(left_name, left_terms), Term::Str(right_name, right_terms)) => {
	    let left_terms_len = left_terms.len();
	    if left_name == right_name && left_terms_len == right_terms.len() {
		let unifications: HashSet<Substitution> = std::iter::zip(left_terms, right_terms)
		    .filter_map(|(l,r)| unify(l, r))
		    .flatten()
		    .collect();
		if unifications.len() == left_terms_len {
		    Some(unifications)
		} else {
		    None
		}
	    } else {
		None
	    }
	}
	 _ => None
    }	
}

#[test]
fn unify_const() {
    let left = Term::Number(56);
    let right = Term::Number(56);
    let result = unify(left, right);
    assert_eq!(Some(HashSet::new()), result);
}

#[test]
fn unify_const_fail() {
    let left = Term::Number(57);
    let right = Term::Number(56);
    let result = unify(left, right);
    assert_eq!(None, result);
}

#[test]
fn unify_const_atom_number() {
    let left = Term::Number(56);
    let right = Term::Atom("esgueva".to_string());
    let result = unify(left, right);
    assert_eq!(None, result);
}

#[test]
fn unify_const_atom() {
    let left = Term::Atom("esgueva".to_string());
    let right = Term::Atom("esgueva".to_string());
    let result = unify(left, right);
    assert_eq!(Some(HashSet::new()), result);
}

#[test]
fn unify_const_atom_fail() {
    let left = Term::Atom("esgueva".to_string());
    let right = Term::Atom("duero".to_string());
    let result = unify(left, right);
    assert_eq!(None, result);
}

#[test]
fn unify_var() {
    let left = Term::Var("X".to_string());
    let right = Term::Atom("esgueva".to_string());
    let result = unify(left, right.clone());
    assert_eq!(Some(HashSet::from([Substitution { name: "X".to_string(), value: right }])), result);
}

#[test]
fn unify_str() {
    let left = Term::Str("f".to_string(), vec![Term::Var("X".to_string()), Term::Number(23), Term::Var("Z".to_string())]);
    let right = Term::Str("f".to_string(), vec![Term::Var("Z".to_string()), Term::Var("Y".to_string()), Term::Var("Y".to_string())]);
    let result = unify(left, right);
    let substitutions = HashSet::from([
	Substitution { name: "X".to_string(), value: Term::Var("Z".to_string()) },
	Substitution { name: "Z".to_string(), value: Term::Var("Y".to_string()) },
	Substitution { name: "Y".to_string(), value: Term::Number(23) }
    ]);
    assert_eq!(Some(substitutions), result);
}
