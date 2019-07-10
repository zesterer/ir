use std::collections::HashMap;

use crate::*;
use crate::lexer::*;
use crate::src::SrcRange;

#[derive(Debug)]
pub struct Program<'a> {
    blocks: HashMap<&'a str, Block<'a>>,
}

#[derive(Debug)]
pub struct Block<'a> {
    tags: Vec<&'a str>, // Identifier list in parens after "BLOCK": BLOCK (param list) ...
    inputs: Vec<Binding<'a>>,
    ops: Vec<(Binding<'a>, Op<'a>)>,
    exit: Branch<'a>,
}

#[derive(Debug)]
pub struct Binding<'a> {
    name: &'a str,
    ty: Type,
}

#[derive(Debug)]
pub enum Type {
    Product(Vec<Type>),
    Sum(Vec<Type>),
    I32,
    Bool,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
    // TODO: More
}

#[derive(Debug)]
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

#[derive(Debug, Clone)]
struct TokenStream<'a> {
    pub tokens: &'a [Token<'a>],
    pub index: usize,
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
    // src: &'a str, // Could be used with token SrcRange info for better errors
    tokens: TokenStream<'a>, // For iterating over the tokens from the lexer
}

/// The type parsers return.
pub type ParserResult<T> = Result<T, Error>;

impl<'src> Parser<'src> {
    pub fn new(/*src: &'src str, */tokens: &'src [Token<'src>]) -> Self {
        Self {/* src, */tokens: TokenStream { tokens, index: 0 } }
    }

    pub fn parse(&mut self) -> ParserResult<Program<'src>> {
        let mut blocks = HashMap::new();

        while let Some(Token(lexeme, range)) = self.tokens.peek(0) {
            if lexeme == &Lexeme::Keyword(Keyword::Block) {
                // TODO: let block = parse_block(tokens)?;
                let (id, b) = self.parse_block()?;
                blocks.insert(id, b);
            } else {
                return Err(error(ErrorKind::ExpectedBlock).at(*range));
            }
        }

        Ok(Program {
            blocks
        })
    }

    fn try_parse<R, F: FnOnce(&mut TokenStream<'src>) -> Option<R>>(t: &mut TokenStream<'src>, f: F) -> Option<R> {
        let mut new_token_iter = t.clone();
        new_token_iter.peek(0)?; // return None if no token up next

        let tree = f(&mut new_token_iter)?;
        *t = new_token_iter;
        Some(tree)
    }

    fn parse_block(&mut self) -> ParserResult<(&'src str, Block<'src>)> {
        // pub struct Block<'a> {
        //     tags: Vec<&'a str>, // Identifier list in parens after "BLOCK": BLOCK (param list) ...
        //     inputs: Vec<Binding<'a>>,
        //     ops: (Binding<'a>, Op<'a>),
        //     exit: Branch<'a>,
        // }
        let name: &'src str;
        let mut tags = Vec::new();
        let mut inputs = Vec::new();
        let mut ops = Vec::new();   // TODO: same here v
        let mut exit = Branch::End; // TODO: don't do this, just for prototyping right now

        match self.tokens.advance() {
            Some(Token(Lexeme::Keyword(Keyword::Block), _)) => {},
            Some(Token(_, range)) => return Err(error(ErrorKind::ExpectedBlock).at(*range)),
            None => return Err(error(ErrorKind::ExpectedBlock)),
        }

        // Possible list of tags in parenthesis after "BLOCK" keyword
        if let Some(Token(Lexeme::Symbol(Symbol::LParen), range)) = self.tokens.peek(0) { // a list of tags has been found
            self.tokens.advance(); // consume (   ... TODO: should we make parenthesis mandatory?
            while let Some(Token(Lexeme::Ident(id), _)) = self.tokens.peek(0) {
                self.tokens.advance();
                tags.push(*id);
            }
            match self.tokens.advance() {
                Some(Token(Lexeme::Symbol(Symbol::RParen), _)) => {}
                Some(Token(_, r)) => return Err(error(ErrorKind::ExpectedRParen{ opening: *range }).at(*r)),
                // TODO: None => return Err(error(ErrorKind::ExpectedRParen(range)))
                _ => unimplemented!(),
            }
        }

        match self.tokens.advance() { // Expect a name
            Some(Token(Lexeme::Ident(id), _)) => name = id,
            Some(Token(_, r)) => return Err(error(ErrorKind::ExpectedIdentifier).at(*r)),
            _ => unimplemented!(),
        }

        match self.tokens.advance() { // Expect (
            Some(Token(Lexeme::Symbol(Symbol::LParen), _)) => {},
            Some(Token(_, r)) => return Err(error(ErrorKind::ExpectedLParen).at(*r)),
            _ => unimplemented!(),
        }

        // Parse parameter list
        let mut id_stack = Vec::<&'src str>::new(); // for chaining id's to types like  (n1, n2, n3 I32)
        while let Some(Token(l, r)) = self.tokens.peek(0) { // TODO: this needs work and is incomplete
            match l {
                Lexeme::Symbol(Symbol::Comma) => {},
                Lexeme::Ident(id) => {
                    self.tokens.advance();
                    id_stack.push(id);
                }
                _ => unimplemented!(), // TODO: error for this?
            }
        }

        Ok((name, Block { tags, inputs, ops, exit }))
    }
}
