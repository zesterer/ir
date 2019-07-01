use std::collections::HashMap;

pub mod lexer;

pub struct Program<'a> {
    blocks: HashMap<&'a str, Block<'a>>,
}

pub struct Block<'a> {
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
