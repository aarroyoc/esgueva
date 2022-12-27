use std::collections::HashMap;
use std::iter::zip;

#[derive(Debug, Clone)]
enum Term {
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

type Bindings = Option<HashMap<String, Term>>;

fn unify(x: Term, y: Term, bindings: Bindings, occurs_check: bool) -> Bindings {
    if x == y {
	bindings
    } else if let Term::Var(var) = x {
	unify_variable(var, y, bindings, occurs_check)
    } else if let Term::Var(var) = y {
	unify_variable(var, x, bindings, occurs_check)
    } else if let (Term::Str(f_x, args_x), Term::Str(f_y, args_y)) = (x, y) {
	if f_x == f_y && args_x.len() == args_y.len() {
	    zip(args_x, args_y).fold(bindings, |acc_bindings, (t_x, t_y)| {
		unify(t_x, t_y, acc_bindings, occurs_check)
	    })
	} else {
	    None
	}
    } else {
	None
    }
}

#[inline]
fn unify_variable(var: String, y: Term, bindings: Bindings, occurs_check_flag: bool) -> Bindings {
    let mut bindings = bindings?;
    match bindings.get(&var) {
	Some(value) => unify(value.clone(), Term::Var(var), Some(bindings), occurs_check_flag),
	None => {
	    if let Term::Var(ref y_var) = y {
		if let Some(y_val) = bindings.get(y_var) {
		    return unify(Term::Var(var), y_val.clone(), Some(bindings), occurs_check_flag)
		}
	    }

	    occurs_check(var.clone(), y.clone(), Some(bindings), occurs_check_flag).and_then(|mut bindings| {
		bindings.insert(var, y.clone());
		Some(bindings)
	    })
	   
	}
    }
}

#[inline]
fn occurs_check(var: String, y: Term, bindings: Bindings, occurs_check_flag: bool) -> Bindings {
    if !occurs_check_flag {
	bindings
    } else if Term::Var(var.clone()) == y {
	None
    } else if let Term::Var(y_var) = y {
	match bindings.clone()?.get(&y_var) {
	    Some(y_val) => {
		return occurs_check(var, y_val.clone(), bindings, occurs_check_flag);
	    }
	    None => bindings
	}
    } else if let Term::Str(_, args) = y {
	args.iter().fold(bindings, |acc_bindings, t_y| {
	    occurs_check(var.clone(), t_y.clone(), acc_bindings, occurs_check_flag)
	})
    } else {
	bindings
    }
}

#[test]
fn unify_atoms() {
    let x = Term::Atom("duero".into());
    let y = Term::Atom("duero".into());
    let bindings = unify(x, y, Some(HashMap::new()), false);
    let expected = Some(HashMap::new());
    assert_eq!(expected, bindings);
}

#[test]
fn unify_atoms_fail() {
    let x = Term::Atom("duero".into());
    let y = Term::Atom("pisuerga".into());
    let bindings = unify(x, y, Some(HashMap::new()), false);
    let expected = None;
    assert_eq!(expected, bindings);
}

#[test]
fn unify_atom_var() {
    let x = Term::Var("River".into());
    let y = Term::Atom("duero".into());
    let bindings = unify(x, y, Some(HashMap::new()), false);
    let mut expected = HashMap::new();
    expected.insert("River".into(), Term::Atom("duero".into()));
    assert_eq!(Some(expected), bindings);
}

#[test]
fn unify_atom_var_2() {
    let y = Term::Var("River".into());
    let x = Term::Atom("duero".into());
    let bindings = unify(x, y, Some(HashMap::new()), false);
    let mut expected = HashMap::new();
    expected.insert("River".into(), Term::Atom("duero".into()));
    assert_eq!(Some(expected), bindings);
}

#[test]
fn unify_var() {
    let x = Term::Var("X".into());
    let y = Term::Var("Y".into());
    let bindings = unify(x, y, Some(HashMap::new()), false);
    let mut expected = HashMap::new();
    expected.insert("X".into(), Term::Var("Y".into()));
    assert_eq!(Some(expected), bindings);
}

#[test]
fn unify_str() {
    let x = Term::Str("f".into(), vec![Term::Var("X".into()), Term::Atom("b".into())]);
    let y = Term::Str("f".into(), vec![Term::Atom("a".into()), Term::Var("Y".into())]);
    let bindings = unify(x, y, Some(HashMap::new()), false);
    let mut expected = HashMap::new();
    expected.insert("X".into(), Term::Atom("a".into()));
    expected.insert("Y".into(), Term::Atom("b".into()));    
    assert_eq!(Some(expected), bindings);
}

#[test]
fn unify_str_fail() {
    let x = Term::Str("f".into(), vec![Term::Var("X".into()), Term::Atom("b".into())]);
    let y = Term::Str("g".into(), vec![Term::Atom("a".into()), Term::Var("Y".into())]);
    let bindings = unify(x, y, Some(HashMap::new()), false);
    assert_eq!(None, bindings);
}

#[test]
fn unify_str_fail_2() {
    let x = Term::Str("f".into(), vec![Term::Var("X".into()), Term::Atom("b".into())]);
    let y = Term::Str("f".into(), vec![Term::Atom("a".into())]);
    let bindings = unify(x, y, Some(HashMap::new()), false);
    assert_eq!(None, bindings);
}

#[test]
fn unify_fxy_norvig_bug() {
    let x = Term::Str("f".into(), vec![Term::Var("X".into()), Term::Var("Y".into())]);
    let y = Term::Str("f".into(), vec![Term::Var("Y".into()), Term::Var("X".into())]);
    let bindings = unify(x, y, Some(HashMap::new()), false);
    let mut expected = HashMap::new();
    expected.insert("X".into(), Term::Var("Y".into()));
    assert_eq!(Some(expected), bindings);
}

#[test]
fn unify_cyclic() {
    let x = Term::Var("X".into());
    let y = Term::Str("f".into(), vec![Term::Var("X".into())]);
    let bindings = unify(x, y, Some(HashMap::new()), true);
    assert_eq!(None, bindings);
}
