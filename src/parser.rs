use std::collections::HashMap;

use crate::*;
use crate::lexer::*;
use crate::src::SrcRange;

pub struct Program<'a> {
    blocks: HashMap<&'a str, Block<'a>>,
}

pub struct Block<'a> {
    tags: Vec<&'a str>, // Identifier list in parens after "BLOCK": BLOCK (param list) ...
    inputs: Vec<Binding<'a>>,
    ops: (Binding<'a>, Op<'a>),
    exit: Branch<'a>,
}

pub struct Binding<'a> {
    name: &'a str,
    ty: Type,
}

pub enum Type {
    Product(Vec<Type>),
    Sum(Vec<Type>),
    I32,
    Bool,
}

pub enum Op<'a> {
    Binary(BinaryOp, Binding<'a>, Binding<'a>),
    Unary(UnaryOp, Binding<'a>),
    ExternCall {
        name: &'a str,
        args: Vec<Binding<'a>>,
    },
    Call {
        name: &'a str,
        args: Vec<Binding<'a>>,
    },
}

pub enum BinaryOp {
	// Math
    Add,
    Sub,
    Mul,
    Div,
    // Comparison
    Eq,
    Neq,
    Geq,
    Leq,
    // TODO: More
}

pub enum UnaryOp {
    Neg,
    Not,
    // TODO: More
}

pub enum Branch<'a> {
    Always(&'a str),
    If {
        predicate: Binding<'a>,
        then_block: &'a str,
        else_block: &'a str,
    },
    Return(Vec<Binding<'a>>),
    End,
}

// TODO: it would be great if we could write our own type for iterating over the tokens

pub fn parse_program<'a, I: std::iter::Iterator<Item=Token<'a>>>(tokens: &mut I) -> Result<Program<'a>, Error> {
    let mut blocks = HashMap::new();

    while let Some(Token(lexeme, range)) = tokens.next() {
        if lexeme == Lexeme::Keyword(Keyword::Block) {
            // TODO: let block = parse_block(tokens)?;
        } else {
            return Err(error(ErrorKind::ExpectedBlock).at(range));
        }
    }

    Ok(Program {
        blocks
    })
}
