use pest::Parser;
use pest_derive::Parser;

fn main() {
    println!("{:?}", 10);
}

#[derive(Parser)]
#[grammar = "auto.pest"]
pub struct AutoParser;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum MonadicVerb {
    // language core
    START,
    SUB,
    CALL,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DyadicVerb {
    // Arithmetic
    ADDITION,       // +
    SUBTRACTION,    // -
    MULTIPLICATION, // -
    DIVISION,       // \
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    // data types
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<AstNode>),

    MonadicOp {
        verb: MonadicVerb,
        operand: Box<AstNode>,
    },
    DyadicOp {
        verb: DyadicVerb,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Terms(Vec<AstNode>),
    Assignment {
        identifier: String,
        expr: Box<AstNode>,
    },
    Identifier(String),
}
