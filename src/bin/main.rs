//! This entire source is used first for testing, and may later be turned into the CLI.

use ir::*;

fn main() {
	let tokens = ir::lexer::lex("one two three");
	println!("{:#?}", tokens);
}
