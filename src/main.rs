use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::collections::HashMap;

use esgueva::database::Database;
use esgueva::parser;
use esgueva::prover;
use esgueva::term::Term;

fn main() {
    println!("Esgueva Prolog 0.1.0 - Adrián Arroyo Calle 2022");
    let args: Vec<String> = env::args().collect();

    match args.len() {
	2 => {
	    if args[1] == "-h" {
		print_help();
	    } else {
		repl(file_to_database(&args[1]))
	    }
	}
	1 => repl(Database::new()),
	_ => print_help()
    }

}

fn print_help() {
    println!("Usage: esgueva [PROLOG FILE]\tStart Esgueva top-level optionally loading a file");
    println!("       esgueva -h\t\tShow help");
}

fn file_to_database(file: &str) -> Database {
    let mut db = Database::new();
    let contents = fs::read_to_string(file).expect("File must exist");

    if let Ok((_, clauses)) = parser::file(&contents) {
	for clause in clauses {
	    db.add_clause(clause);
	}
    } else {
	eprintln!("Error loading file: {}", file);
    }
    
    db
}

fn repl(database: Database) {
    loop {
	print!("?- ");
	io::stdout().flush().unwrap();
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	if let Ok((_, mut goals)) = parser::clause_body(&input) {
	    let vars_in_goals = prover::find_variables_in_goals(&goals);
	    goals.push(Term::Atom("__backtracking?".into()));
	    prover::prove_all(goals.into(), Some(HashMap::new()), &database, &vars_in_goals);
	    println!("false.");
	} else {
	    eprintln!("Can't parse query!");
	}
    }
}
