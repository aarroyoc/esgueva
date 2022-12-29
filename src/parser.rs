use nom::{
    IResult,
    Err,
    error::Error,
    error::ErrorKind,
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    character::complete::char,
    character::complete::alphanumeric0,
    character::complete::multispace1,
    multi::many0,
    multi::many1,
    multi::separated_list0,
    multi::separated_list1,
};

use crate::term::Term;
use crate::database::Clause;

pub fn file(input: &str) -> IResult<&str, Vec<Clause>> {
    let (input, clauses) = separated_list0(multispace1, clause)(input)?;

    Ok((input, clauses))
}

fn clause(input: &str) -> IResult<&str, Clause> {
    let (input, clause) = alt((clause_fact, clause_rule))(input)?;

    Ok((input, clause))
}

fn clause_fact(input: &str) -> IResult<&str, Clause> {
    let (input, head) = alt((term_str, term_atom))(input)?;
    let (input, _) = char('.')(input)?;

    Ok((input, Clause { head, body: vec![] }))
}

fn clause_rule(input: &str) -> IResult<&str, Clause> {
    let (input, head) = alt((term_str, term_atom))(input)?;
    let (input, _) = many1(char(' '))(input)?;
    let (input, _) = tag(":-")(input)?;
    let (input, _) = many1(char(' '))(input)?;
    let (input, body) = clause_body(input)?;

    Ok((input, Clause { head, body }))
}

pub fn clause_body(input: &str) -> IResult<&str, Vec<Term>> {
    let (input, goals) = separated_list1(spaced_comma, alt((term_str, term_atom)))(input)?;
    let (input, _) = char('.')(input)?;

    Ok((input, goals))
}

fn spaced_comma(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(char(' '))(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = many0(char(' '))(input)?;

    Ok((input, ()))
}

fn term_str(input: &str) -> IResult<&str, Term> {
    let (input, first) = anychar(input)?;
    if !first.is_ascii_lowercase() {
	return Err(Err::Error(Error::new(input, ErrorKind::Char)));
    }
    let (input, atom) = alphanumeric0(input)?;
    
    let (input, _) = char('(')(input)?;

    let (input, args) = separated_list1(spaced_comma, alt((term_str, term_var, term_atom)))(input)?;
    
    let (input, _) = char(')')(input)?;

    Ok((input, Term::Str(format!("{}{}", first, atom), args)))
}

fn term_var(input: &str) -> IResult<&str, Term> {
    let (input, first) = anychar(input)?;
    if !first.is_ascii_uppercase() {
	return Err(Err::Error(Error::new(input, ErrorKind::Char)));
    }
    let (input, var) = alphanumeric0(input)?;

    Ok((input, Term::Var(format!("{}{}", first, var))))
}

fn term_atom(input: &str) -> IResult<&str, Term> {
    let (input, first) = anychar(input)?;
    if !first.is_ascii_lowercase() {
	return Err(Err::Error(Error::new(input, ErrorKind::Char)));
    }
    let (input, atom) = alphanumeric0(input)?;

    Ok((input, Term::Atom(format!("{}{}", first, atom))))
}

#[test]
fn parse1() {
    let input = "f(X,b,g(T)), g(X, a, Z).";
    let result = clause_body(input);
    let expected = vec![
	Term::Str("f".into(), vec![Term::Var("X".into()), Term::Atom("b".into()), Term::Str("g".into(), vec![Term::Var("T".into())])]),
	Term::Str("g".into(), vec![Term::Var("X".into()), Term::Atom("a".into()), Term::Var("Z".into())]),
    ];
    assert_eq!(result, Ok(("", expected)));
}

#[test]
fn parse_fact() {
    let input = "f(adrian, valladolid).";
    let result = clause(input);
    let expected = Clause {
	head: Term::Str("f".into(), vec![Term::Atom("adrian".into()), Term::Atom("valladolid".into())]),
	body: vec![],
    };
    assert_eq!(result, Ok(("", expected)));
}

#[test]
fn parse_rule() {
    let input = "likes(X, sandy) :- likes(X, cats), likes(X, kim).";
    let result = clause(input);
    let expected = Clause {
	head: Term::Str("likes".into(), vec![Term::Var("X".into()), Term::Atom("sandy".into())]),
	body: vec![
	    Term::Str("likes".into(), vec![Term::Var("X".into()), Term::Atom("cats".into())]),
	    Term::Str("likes".into(), vec![Term::Var("X".into()), Term::Atom("kim".into())]),
	],
    };
    assert_eq!(result, Ok(("", expected)));
}

#[test]
fn parse_file() {
    let input = include_str!("example.pl");
    let result = file(input);
    let expected = vec![
	Clause {
	    head: Term::Str("likes".into(), vec![Term::Atom("kim".into()), Term::Atom("robin".into())]),
	    body: vec![],
	},
	Clause {
	    head: Term::Str("likes".into(), vec![Term::Atom("sandy".into()), Term::Atom("lee".into())]),
	    body: vec![],
	},
	Clause {
	    head: Term::Str("likes".into(), vec![Term::Atom("sandy".into()), Term::Atom("kim".into())]),
	    body: vec![],
	},
	Clause {
	    head: Term::Str("likes".into(), vec![Term::Atom("robin".into()), Term::Atom("cats".into())]),
	    body: vec![],
	},
	Clause {
	    head: Term::Str("likes".into(), vec![Term::Atom("sandy".into()), Term::Var("X".into())]),
	    body: vec![
		Term::Str("likes".into(), vec![Term::Var("X".into()), Term::Atom("cats".into())])
	    ],
	},
	Clause {
	    head: Term::Str("likes".into(), vec![Term::Atom("kim".into()), Term::Var("X".into())]),
	    body: vec![
		Term::Str("likes".into(), vec![Term::Var("X".into()), Term::Atom("lee".into())]),
		Term::Str("likes".into(), vec![Term::Var("X".into()), Term::Atom("kim".into())])		 
	    ],
	},
	Clause {
	    head: Term::Str("likes".into(), vec![Term::Var("X".into()), Term::Var("X".into())]),
	    body: vec![],
	}
    ];
    assert_eq!(result, Ok(("\n", expected)));
}
