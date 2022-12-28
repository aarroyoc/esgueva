use uuid::Uuid;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::term::Term;
use crate::unify::{Bindings, unify};
use crate::database::{Database, Predicate, Clause};

fn prove(goal: Term, bindings: Bindings, database: &Database) -> Option<Vec<Bindings>> {
    if let Some(predicate) = Predicate::from_term(&goal) {
	if let Some(clauses) = database.get_clauses(&predicate) {
	    let mut solutions = Vec::new();
	    for clause in clauses {
		let renamed_clause = rename_variables(&clause);
		let bindings = unify(goal.clone(), renamed_clause.head, bindings.clone(), false);
		if bindings.is_none() {
		    // do nothing
		} else if renamed_clause.body.len() == 0 {
		    solutions.push(bindings);
		} else {
		    if let Some(mut all_solutions) = prove_all(renamed_clause.body, vec![bindings], database) {
			solutions.append(&mut all_solutions);
		    }
		}
	    }
	    if solutions.len() == 0 {
		None
	    } else {
		Some(solutions)
	    }
	} else {
	    None
	}
    } else {
	None
    }
}

fn prove_all(mut goals: Vec<Term>, bindings: Vec<Bindings>, database: &Database) -> Option<Vec<Bindings>> {
    if let Some(goal) = goals.pop() {
	let mut solutions = Vec::new();
	for binding in bindings {
	    if let Some(mut goal_solutions) = prove(goal.clone(), binding, database) {
		solutions.append(&mut goal_solutions);
	    }
	}
	if solutions.len() == 0 {
	    None
	} else {
	    prove_all(goals, solutions, database)
	}
    } else {
	Some(bindings)
    }
}

fn rename_variables(clause: &Clause) -> Clause {
    let mut bindings = HashMap::new();
    Clause {
	head: rename_term(&clause.head, &mut bindings),
	body: clause.body.iter().map(|term| rename_term(term, &mut bindings)).collect(),
    }
}

fn rename_term(term: &Term, bindings: &mut HashMap<String, String>) -> Term {
    match term {
	Term::Atom(f) => Term::Atom(f.clone()),
	Term::Var(var) => {
	    if let Some(subst) = bindings.get(var) {
		Term::Var(subst.clone())
	    } else {
		let id = Uuid::now_v1(&[1, 2, 3, 4, 5, 6]).to_string();
		bindings.insert(var.clone(), id.clone());
		Term::Var(id)
	    }
	},
	Term::Str(f, args) => {
	    Term::Str(f.clone(), args.iter().map(|arg| rename_term(arg, bindings)).collect())
	}
    }
}

fn top_level_prove(goals: Vec<Term>, database: &Database) -> String {
    let vars_in_goals = find_variables_in_goals(&goals);
    let solutions = prove_all(goals, vec![Some(HashMap::new())], database);

    if let Some(solutions) = solutions {
	let mut output = Vec::new();
	for solution in solutions {
	    let mut line = Vec::new();
	    for var in &vars_in_goals {
		line.push(format!("{} = {}", var, subst_bindings(solution.clone(), Term::Var(var.clone()))));
	    }
	    output.push(line.join(","));
	}
	output.join(";\n")
    } else {
	format!("false.")
    }
}

fn subst_bindings(bindings: Bindings, term: Term) -> Term {
    let bindings = bindings.expect("Only can be called when bindings are OK");
    match term {
	Term::Atom(x) => Term::Atom(x.clone()),
	Term::Var(ref x) => {
	    if let Some(value) = bindings.get(x) {
		subst_bindings(Some(bindings.clone()), value.clone())
	    } else {
		Term::Var(x.clone())
	    }
	}
	Term::Str(f, args) => {
	    Term::Str(f.clone(), args.iter().map(|t| subst_bindings(Some(bindings.clone()), t.clone())).collect())
	}
    }
}

fn find_variables_in_goals(goals: &Vec<Term>) -> HashSet<String> {
    let mut vars = HashSet::new();
    for goal in goals {
	match goal {
	    Term::Var(var) => {
		vars.insert(var.clone());
	    },
	    Term::Atom(_) => (),
	    Term::Str(_, args) => vars.extend(find_variables_in_goals(&args))
	}
    }
    vars
}

#[test]
fn socrates_test() {
    let mut db = Database::new();
    let clause = Clause {
	head: Term::Str("mortal".into(), vec![Term::Var("X".into())]),
	body: vec![Term::Str("human".into(), vec![Term::Var("X".into())])],
    };
    db.add_clause(clause);
    let clause = Clause {
	head: Term::Str("human".into(), vec![Term::Atom("socrates".into())]),
	body: vec![Term::Atom("true".into())],
    };
    db.add_clause(clause);

    let clause = Clause {
	head: Term::Atom("true".into()),
	body: Vec::new(),
    };
    db.add_clause(clause);

    let query1 = Term::Str("human".into(), vec![Term::Atom("socrates".into())]);
    let result = top_level_prove(vec![query1], &db);
    assert_eq!(result, "");

    let query2 = Term::Str("mortal".into(), vec![Term::Atom("socrates".into())]);
    let result = top_level_prove(vec![query2], &db);
    assert_eq!(result, "");

    let query3 = Term::Str("mortal".into(), vec![Term::Var("X".into())]);
    let result = top_level_prove(vec![query3], &db);
    assert_eq!(result, "X = socrates");

    let query4 = Term::Str("mrtl".into(), vec![Term::Atom("socrates".into())]);
    let result = top_level_prove(vec![query4], &db);
    assert_eq!(result, "false.");

    let query5 = Term::Str("mortal".into(), vec![Term::Atom("gepeto".into())]);
    let result = top_level_prove(vec![query5], &db);
    assert_eq!(result, "false.");
}

#[test]
fn likes() {
    let mut db = Database::new();
    db.add_clause(Clause {
	head: Term::Str("likes".into(), vec![Term::Atom("kim".into()), Term::Atom("robin".into())]),
	body: vec![],
    });
    db.add_clause(Clause {
	head: Term::Str("likes".into(), vec![Term::Atom("sandy".into()), Term::Atom("lee".into())]),
	body: vec![],
    });
    db.add_clause(Clause {
	head: Term::Str("likes".into(), vec![Term::Atom("sandy".into()), Term::Atom("kim".into())]),
	body: vec![],
    });
    db.add_clause(Clause {
	head: Term::Str("likes".into(), vec![Term::Atom("robin".into()), Term::Atom("cats".into())]),
	body: vec![],
    });
    db.add_clause(Clause {
	head: Term::Str("likes".into(), vec![Term::Atom("sandy".into()), Term::Var("X".into())]),
	body: vec![Term::Str("likes".into(), vec![Term::Var("X".into()), Term::Atom("cats".into())])],
    });
    db.add_clause(Clause {
	head: Term::Str("likes".into(), vec![Term::Atom("kim".into()), Term::Var("X".into())]),
	body: vec![
	    Term::Str("likes".into(), vec![Term::Var("X".into()), Term::Atom("lee".into())]),
	    Term::Str("likes".into(), vec![Term::Var("X".into()), Term::Atom("kim".into())])
	],
    });
    db.add_clause(Clause {
	head: Term::Str("likes".into(), vec![Term::Var("X".into()), Term::Var("X".into())]),
	body: vec![],
    });

    let query1 = Term::Str("likes".into(), vec![Term::Atom("sandy".into()), Term::Var("Who".into())]);
    let result = top_level_prove(vec![query1], &db);
    assert_eq!(result, "Who = lee;\nWho = kim;\nWho = robin;\nWho = sandy;\nWho = cats;\nWho = sandy");

    let query2 = Term::Str("likes".into(), vec![Term::Var("Who".into()), Term::Atom("sandy".into())]);
    let result = top_level_prove(vec![query2], &db);
    assert_eq!(result, "Who = sandy;\nWho = kim;\nWho = sandy");

    let query3 = Term::Str("likes".into(), vec![Term::Atom("robin".into()), Term::Atom("lee".into())]);
    let result = top_level_prove(vec![query3], &db);
    assert_eq!(result, "false.");
}
