use crate::src::{SrcLoc, SrcRange};

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a>(pub Lexeme<'a>, pub SrcRange);

#[derive(Debug, Clone, PartialEq)]
pub enum Lexeme<'a> {
	Identifier(&'a str),
	Str(&'a str),
	Num(&'a str),
	Keyword(Keyword),
	Symbol(Symbol),
	Operator(Operator),
	// TODO: make token types
}
use Lexeme::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
	Block,
	// TODO: more keywords
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
	LParen,
	RParen,
	Colon,
	Comma,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
	// TODO: operators
}

pub fn lex<'a>(src: &'a str) -> Result<Vec<Token<'a>>, ()> {
	let mut tokens = Vec::<Token>::new();
	let mut src_loc = SrcLoc::new(1, 1);

	let mut chars = src.chars().enumerate().peekable();
	while let Some((i, c)) = chars.next() {
		match c {
			'\n' => {
				src_loc.line += 1;
				src_loc.col = 1;
				continue; // Don't want src_loc.col += 1;
			}

			// Symbols
			'(' => tokens.push(Token(Symbol(Symbol::LParen), SrcRange::new(src_loc, 1))),
			')' => tokens.push(Token(Symbol(Symbol::RParen), SrcRange::new(src_loc, 1))),
			':' => tokens.push(Token(Symbol(Symbol::Colon), SrcRange::new(src_loc, 1))),
			',' => tokens.push(Token(Symbol(Symbol::Comma), SrcRange::new(src_loc, 1))),
			
			// Operators
			
			// TODO: matching operators and other single- or double-wide tokens
			
			_ => {
				if c.is_whitespace() {
					// Don't do anything, but don't 'continue' so we can src_loc.col += 1;
				} else if c.is_digit(10) {
					// TODO: lexing numbers
				} else if c.is_alphabetic() { // identifiers & keywords
					let start_index = i; // starting index in the src string slice
					let mut range = SrcRange::new(src_loc, 1); // region of the slice we're making
					
					while let Some((_, c)) = chars.peek() {
						if c.is_alphabetic() || c == &'_' {
							chars.next(); // consume the character
							range.len += 1;
							src_loc.col += 1;
						} else {
							break;
						}
					}
					
					tokens.push(Token(Identifier(&src[start_index..start_index+range.len]), range));
				} else {
					return Err(()); // TODO: real lexer errors
				}
			}
		}
		src_loc.col += 1;
	}
	
	Ok(tokens)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn simple_lex() {
		let src = "( abc ):";
		let tokens = lex(src).unwrap();
		assert_eq!(tokens, vec![
			Token(Lexeme::Symbol(Symbol::LParen),  SrcRange::new(SrcLoc::new(1, 1), 1)), // (
			Token(Lexeme::Identifier(&src[2..=4]), SrcRange::new(SrcLoc::new(1, 3), 3)), // abc
			Token(Lexeme::Symbol(Symbol::RParen),  SrcRange::new(SrcLoc::new(1, 7), 1)), // )
			Token(Lexeme::Symbol(Symbol::Colon),   SrcRange::new(SrcLoc::new(1, 8), 1)), // :
		]);
	}

	#[test]
	fn src_pos() {
		let src = " this
is
  a test
";
		let tokens = lex(src).unwrap();
		assert_eq!(tokens, vec![
			Token(Lexeme::Identifier(&src[1..=4]), SrcRange::new(SrcLoc::new(1, 2), 4)), // this
			Token(Lexeme::Identifier(&src[6..=7]), SrcRange::new(SrcLoc::new(2, 1), 2)), // is
			Token(Lexeme::Identifier(&src[11..=11]), SrcRange::new(SrcLoc::new(3, 3), 1)), // a
			Token(Lexeme::Identifier(&src[13..=16]), SrcRange::new(SrcLoc::new(3, 5), 4)), // test
		]);
	}
}
