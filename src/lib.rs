use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Number(i64),
    Atom(String),
    Str(String, Vec<Term>),
    Var(String),
}

pub fn unify(left: Term, right: Term) -> Option<HashMap<String, Term>> {
    match (left, right) {
	(Term::Number(x), Term::Number(y)) => {
	    if x == y {
		Some(HashMap::new())
	    } else {
		None
	    }
	},
	(Term::Atom(x), Term::Atom(y)) => {
	    if x == y {
		Some(HashMap::new())
	    } else {
		None
	    }
	}
	(Term::Var(name), value) => {
	    Some(HashMap::from([(name, value)]))
	}
	(value, Term::Var(name)) => {
	    Some(HashMap::from([(name, value)]))
	}
	(Term::Str(left_name, left_terms), Term::Str(right_name, right_terms)) => {
	    let left_terms_len = left_terms.len();
	    if left_name == right_name && left_terms_len == right_terms.len() {
		let unifications: Vec<HashMap<String, Term>> = std::iter::zip(left_terms, right_terms)
		    .filter_map(|(l,r)| unify(l, r))
		    .collect();
		if unifications.len() == left_terms_len {
		    let mut final_unification: HashMap<String, Term> = HashMap::new();
		    for unification in unifications {
			for (var, term) in &unification {
			    if final_unification.contains_key(var) {
				let value = final_unification[var].clone();
				if *term == value {
				    continue;
				} else if let Term::Var(v) = value {
				    final_unification.remove(var).unwrap();
				    final_unification.insert(var.clone(), term.clone());
				    final_unification.insert(v.clone(), Term::Var(var.clone()));
			        } else {
				    return None;
				}
			    } else {
				final_unification.insert(var.clone(), term.clone());
			    }
			}
		    }
		    Some(final_unification)
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
    assert_eq!(Some(HashMap::new()), result);
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
    assert_eq!(Some(HashMap::new()), result);
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
    assert_eq!(Some(HashMap::from([("X".to_string(),right )])), result);
}

#[test]
fn unify_str() {
    let left = Term::Str("f".to_string(), vec![Term::Var("X".to_string()), Term::Number(23), Term::Var("Z".to_string())]);
    let right = Term::Str("f".to_string(), vec![Term::Var("Z".to_string()), Term::Var("Y".to_string()), Term::Var("Y".to_string())]);
    let result = unify(left, right);
    let substitutions = HashMap::from([
	("X".to_string(), Term::Var("Z".to_string())),
	("Z".to_string(), Term::Var("Y".to_string())),
	("Y".to_string(), Term::Number(23))
    ]);
    assert_eq!(Some(substitutions), result);
}

#[test]
fn unify_str_2() {
    let left = Term::Str("f".to_string(), vec![Term::Var("X".to_string()), Term::Number(4)]);
    let right = Term::Str("f".to_string(), vec![Term::Number(4), Term::Var("X".to_string())]);
    let result = unify(left, right);
    let substitutions = HashMap::from([
	("X".to_string(), Term::Number(4))
    ]);
    assert_eq!(Some(substitutions), result);    
}

#[test]
fn unify_str_3() {
    let left = Term::Str("f".to_string(), vec![Term::Var("X".to_string()), Term::Number(4)]);
    let right = Term::Str("f".to_string(), vec![Term::Number(5), Term::Var("X".to_string())]);
    let result = unify(left, right);
    assert_eq!(None, result);    
}

#[test]
fn unify_str_4() {
    let left = Term::Str("f".to_string(), vec![Term::Var("X".to_string()), Term::Var("X".to_string())]);
    let right = Term::Str("f".to_string(), vec![Term::Var("Y".to_string()), Term::Number(4)]);
    let result = unify(left, right);
    let substitutions = HashMap::from([
	("X".to_string(), Term::Number(4)),
	("Y".to_string(), Term::Var("X".to_string())),
    ]);
    assert_eq!(Some(substitutions), result);    
}

#[test]
fn unify_str_5() {
    let left = Term::Str("f".to_string(), vec![Term::Var("X".to_string()), Term::Var("Y".to_string()), Term::Var("Y".to_string())]);
    let right = Term::Str("f".to_string(), vec![Term::Var("Y".to_string()), Term::Var("X".to_string()), Term::Var("Y".to_string())]);
    let result = unify(left, right);
    let substitutions = HashMap::from([
	("X".to_string(), Term::Var("Y".to_string())),
	("Y".to_string(), Term::Var("Y".to_string())),
    ]);
    assert_eq!(Some(substitutions), result);    
}

#[test]
fn unify_str_6() {
    let left = Term::Str("f".to_string(), vec![Term::Var("X".to_string()), Term::Var("Y".to_string()), Term::Var("Y".to_string())]);
    let right = Term::Str("f".to_string(), vec![Term::Var("Y".to_string()), Term::Var("X".to_string()), Term::Var("X".to_string())]);
    let result = unify(left, right);
    let substitutions = HashMap::from([
	("X".to_string(), Term::Var("Y".to_string())),
	("Y".to_string(), Term::Var("X".to_string())),
    ]);
    assert_eq!(Some(substitutions), result);    
}

#[test]
fn unify_str_7() {
    let left = Term::Str("f".to_string(), vec![Term::Str("g".to_string(), vec![Term::Number(4)])]);
    let right = Term::Str("f".to_string(), vec![Term::Str("g".to_string(), vec![Term::Var("X".to_string())])]);
    let result = unify(left, right);
    let substitutions = HashMap::from([
	("X".to_string(), Term::Number(4)),
    ]);
    assert_eq!(Some(substitutions), result);    
}
