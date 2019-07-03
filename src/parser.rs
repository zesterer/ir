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

struct TokenStream<'a> {
    pub(crate) tokens: &'a [Token<'a>],
    pub(crate) index: usize,
}

impl<'a> TokenStream<'a> {
    pub fn advance(&mut self) -> Option<&'a Token<'a>> {
        self.index += 1;
        self.tokens.get(self.index-1)
    }
    // .peek(0) should get the next unconsumed token
    pub fn peek(&self, index: usize) -> Option<&'a Token<'a>> {
        self.tokens.get(self.index+index)
    }
}

pub struct Parser<'a> {
    src: &'a str, // Could be used with token SrcRange info for better errors
    tokens: TokenStream<'a>, // For iterating over the tokens from the lexer
}

impl<'src> Parser<'src> {
    pub fn new(src: &'src str, tokens: &'src [Token<'src>]) -> Self {
        Self { src, tokens: TokenStream { tokens, index: 0 } }
    }

    pub fn parse(&mut self) -> Result<Program<'src>, Error> {
        let mut blocks = HashMap::new();

        while let Some(Token(lexeme, range)) = self.tokens.advance() {
            if lexeme == &Lexeme::Keyword(Keyword::Block) {
                // TODO: let block = parse_block(tokens)?;
            } else {
                return Err(error(ErrorKind::ExpectedBlock).at(*range));
            }
        }

        Ok(Program {
            blocks
        })
    }
}
