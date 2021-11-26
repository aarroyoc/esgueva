#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "prolog.pest"]
struct PrologParser;

#[derive(Debug, Eq, PartialEq)]
struct Ast<'a> {
    rules: Vec<AstRule<'a>>
}

#[derive(Debug, Eq, PartialEq)]
struct AstRule<'a> {
    head: AstTerm<'a>
}

#[derive(Debug, Eq, PartialEq)]
enum AstTerm<'a> {
    Atom(&'a str),
    Number(i64),
    Structure(&'a str, Vec<AstTerm<'a>>),
    Var(&'a str)
}

fn parse_prolog(file: &str) -> Result<Ast, Error<Rule>> {
    let prolog_file = PrologParser::parse(Rule::prolog, file)?.next().unwrap();

    fn parse_term(pair: Pair<Rule>) -> AstTerm {
        let term_pair = pair.into_inner().next().unwrap();
        match term_pair.as_rule() {
            Rule::atom => {
                AstTerm::Atom(term_pair.as_str())
            },
            Rule::number => {
                AstTerm::Number(term_pair.as_str().parse().unwrap())
            },
            Rule::variable => {
                AstTerm::Var(term_pair.as_str())
            },
            Rule::structure => {
                let mut inner = term_pair.into_inner();
                let functor = inner.next().unwrap().as_str();
                let mut terms = Vec::new();
                while let Some(term) = inner.next() {
                    terms.push(parse_term(term));
                }
                AstTerm::Structure(functor, terms)
            }
            _ => {
                unreachable!();
            }
        }
    }

    fn parse_rule(pair: Pair<Rule>) -> Option<AstRule> {
        if pair.as_rule() == Rule::EOI {
            return None;
        }
        Some(AstRule {
            head: parse_term(pair.into_inner().next().unwrap())
        })
    }

    let ast = Ast {
        rules: prolog_file.into_inner().filter_map(parse_rule).collect()
    };

    Ok(ast)
}


#[cfg(test)]
mod tests {
    use crate::{parse_prolog, Ast, AstRule, AstTerm};

    #[test]
    fn parse_rule_atom() {
        let code = "a.";
        let result = parse_prolog(code);
        let ast = Ast {
            rules: vec![AstRule {
                head: AstTerm::Atom("a")
            }]
        };
        assert_eq!(Ok(ast), result);
    }

    #[test]
    fn parse_rule_atom_large() {
        let code = "amigos_de_prolog.";
        let result = parse_prolog(code);
        let ast = Ast {
            rules: vec![AstRule {
                head: AstTerm::Atom("amigos_de_prolog")
            }]
        };
        assert_eq!(Ok(ast), result);
    }

    #[test]
    fn parse_rule_structure() {
        let code = "f(a, b, c).";
        let result = parse_prolog(code);
        let ast = Ast {
            rules: vec![AstRule {
                head: AstTerm::Structure("f", vec![AstTerm::Atom("a"), AstTerm::Atom("b"), AstTerm::Atom("c")])
            }]
        };
        assert_eq!(Ok(ast), result);
    }

    #[test]
    fn parse_rule_structure_complex() {
        let code = "f(a, g(b), c).";
        let result = parse_prolog(code);
        let ast = Ast {
            rules: vec![AstRule {
                head: AstTerm::Structure("f", vec![AstTerm::Atom("a"), AstTerm::Structure("g", vec![AstTerm::Atom("b")]), AstTerm::Atom("c")])
            }]
        };
        assert_eq!(Ok(ast), result);
    }

    #[test]
    fn parse_number() {
        let code = "age(42).";
        let result = parse_prolog(code);
        let ast = Ast {
            rules: vec![AstRule {
                head: AstTerm::Structure("age", vec![AstTerm::Number(42)])
            }]
        };
        assert_eq!(Ok(ast), result);
    }

    #[test]
    fn parse_variable() {
        let code = "name(Name).";
        let result = parse_prolog(code);
        let ast = Ast {
            rules: vec![AstRule {
                head: AstTerm::Structure("name", vec![AstTerm::Var("Name")])
            }]
        };
        assert_eq!(Ok(ast), result);
    }
}
