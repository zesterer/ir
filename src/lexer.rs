use crate::*;
use crate::src::{SrcLoc, SrcRange};

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a>(pub Lexeme<'a>, pub SrcRange);

#[derive(Debug, Clone, PartialEq)]
pub enum Lexeme<'a> {
	Ident(&'a str),
	Str(&'a str),
	Int(&'a str),
	Float(&'a str),
	Keyword(Keyword),
	Symbol(Symbol),
	Operator(Operator),
	EOL, EOF,
	// TODO: make token types
}
use Lexeme::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
	Block,	   // BLOCK
	BranchEnd, // branch_end
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
	Equals,
	// TODO: operators
}

pub fn lex<'a>(src: &'a str) -> Result<Vec<Token<'a>>, Error> {
	let mut tokens = Vec::<Token>::new();
	let mut src_loc = SrcLoc::new(1, 1);

	// Helper macro for keeping the single-token lexing simple.
	macro_rules! push_single {
		($token:expr) => {
			tokens.push(Token($token, SrcRange::new(src_loc, 1)))
		};
	}

	let mut chars = src.chars().enumerate().peekable();
	while let Some((i, c)) = chars.next() {
		match c {
			// Control
			'\n' => {
				push_single!(EOL);
				src_loc.line += 1;
				src_loc.col = 1;
				continue; // Don't want src_loc.col += 1;
			}

			';' => { // Skip comments
				while let Some((_, c)) = chars.peek() {
					if c != &'\n' {
						chars.next();
						src_loc.col += 1;
					} else {
						break; // leave the '\n' for next cycle
					}
				}
			}

			// Symbols
			'(' => push_single!(Symbol(Symbol::LParen)),
			')' => push_single!(Symbol(Symbol::RParen)),
			':' => push_single!(Symbol(Symbol::Colon)),
			',' => push_single!(Symbol(Symbol::Comma)),
			
			// Operators
			'=' => push_single!(Operator(Operator::Equals)),
			
			// TODO: matching operators and other single- or double-wide tokens
			
			_ => {
				if c.is_whitespace() {
					// Don't do anything, but don't 'continue' so we can src_loc.col += 1;
				} else if c.is_digit(10) {
					let start_index = i; // starting index in the src number slice
					let mut range = SrcRange::new(src_loc, 1); // region of the slice we're making
					let mut is_float = false;
					
					while let Some((_, c)) = chars.peek() {
						if c.is_digit(10) {
							chars.next(); // consume the character
							range.len += 1;
							src_loc.col += 1;
						} else if c == &'.' {
							let (_, c) = chars.next().unwrap(); // consume '.'
							range.len += 1;
							src_loc.col += 1;

							// "if we already had a '.', then return an error"
							if (&src[start_index..start_index+range.len-1]).contains(".") {
								return Err(error(ErrorKind::UnexpectedChar(c)).at(
									SrcRange::new(src_loc, 1)
								));
							}

							is_float = true;
						} else {
							break;
						}
					}

					let slice = &src[start_index..start_index+range.len];
					if is_float {
						tokens.push(Token(Float(slice), range));
					} else {
						tokens.push(Token(Int(slice), range));
					}
				} else if c == '_' || c.is_alphabetic() { // identifiers & keywords
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

					let slice = &src[start_index..start_index+range.len];
					match slice {
						// Keywords
						"BLOCK" => tokens.push(Token(Keyword(Keyword::Block), range)),
						"branch_end" => tokens.push(Token(Keyword(Keyword::BranchEnd), range)),

						"_" => unimplemented!(), // TODO: support this

						// Identifier
						_ => tokens.push(Token(Ident(slice), range)),
					}
				} else {
					return Err(error(ErrorKind::UnexpectedChar(c)).at(SrcRange::new(src_loc, 1)));
				}
			}
		}
		src_loc.col += 1;
	}
	tokens.push(Token(EOF, SrcRange::new(src_loc, 0))); // NOTE: EOF has len 0
	
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
			Token(Lexeme::Ident(&src[2..=4]), SrcRange::new(SrcLoc::new(1, 3), 3)), // abc
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
			Token(Lexeme::Ident(&src[1..=4]), SrcRange::new(SrcLoc::new(1, 2), 4)), // this
			Token(Lexeme::Ident(&src[6..=7]), SrcRange::new(SrcLoc::new(2, 1), 2)), // is
			Token(Lexeme::Ident(&src[11..=11]), SrcRange::new(SrcLoc::new(3, 3), 1)), // a
			Token(Lexeme::Ident(&src[13..=16]), SrcRange::new(SrcLoc::new(3, 5), 4)), // test
		]);
	}
}
