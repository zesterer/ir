#[derive(Debug, Clone)]
pub enum Token<'a> {
	Identifier(&'a str),
	Keyword(Keyword),
	Symbol(Symbol),
	Operator(Operator),
	// TODO: make token types
}

#[derive(Debug, Clone)]
pub enum Keyword {
	Block,
	// TODO: more keywords
}

#[derive(Debug, Clone)]
pub enum Symbol {
	LParen,
	RParen,
	Colon,
	Comma,
}

#[derive(Debug, Clone)]
pub enum Operator {
	// TODO: operators
}

pub fn lex<'a>(src: &'a str) -> Result<Vec<Token<'a>>, ()> {
	let mut tokens = Vec::<Token>::new();

	let mut chars = src.chars().enumerate().peekable();
	while let Some((i, c)) = chars.next() {
		match c {
			// Symbols
			'(' => tokens.push(Token::Symbol(Symbol::LParen)),
			')' => tokens.push(Token::Symbol(Symbol::RParen)),
			':' => tokens.push(Token::Symbol(Symbol::Colon)),
			',' => tokens.push(Token::Symbol(Symbol::Comma)),
			
			// Operators
			
			// TODO: matching operators and other single- or double-wide tokens
			
			_ => {
				if c.is_whitespace() {
					continue;
				} else if c.is_digit(10) {
					// TODO: lexing numbers
				} else if c.is_alphabetic() { // identifiers & keywords
					let start_index = i; // starting index in the src string slice
					let mut end_index = i; // end of the slice we're making
					
					while let Some((_, c)) = chars.peek() {
						if c.is_alphabetic() || c == &'_' {
							chars.next(); // consume the character
							end_index += 1;
						} else {
							break;
						}
					}
					
					tokens.push(Token::Identifier(&src[start_index..=end_index]));
				} else {
					return Err(()); // TODO: real lexer errors
				}
			}
		}
	}
	
	Ok(tokens)
}
