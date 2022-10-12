use pest::iterators::Pairs;
use pest::Parser;

pub mod ast;

use self::ast::Node;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct BfParser;

pub fn parse(source: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    let result = BfParser::parse(Rule::file, source)?;

    Ok(result)
}

pub fn construct_ast(pairs: Pairs<Rule>) -> Vec<Node> {
    let mut ast = vec![];

    for pair in pairs {
        match pair.as_rule() {
            Rule::file => {
                return construct_ast(pair.into_inner());
            }
            Rule::program => {
                return construct_ast(pair.into_inner());
            }
            Rule::forward => ast.push(Node::Forward),
            Rule::backward => ast.push(Node::Backward),
            Rule::increment => ast.push(Node::Increment),
            Rule::decrement => ast.push(Node::Decrement),
            Rule::input => ast.push(Node::Input),
            Rule::output => ast.push(Node::Output),
            Rule::while_loop => ast.push(Node::Loop {
                children: construct_ast(pair.into_inner().clone()),
            }),
            _ => {
                debug_assert!(false, "Unmatched {:?}\n", pair.as_rule());
            }
        }
    }

    ast
}
