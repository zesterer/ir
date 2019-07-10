//! This entire source is used first for testing, and may later be turned into the CLI.

use ir::*;
use std::env::args;
use std::fs::read_to_string;

fn main() {
	let argv = args().collect::<Vec<String>>();
	if argv.len() > 1 {
		let src = read_to_string(&argv[1]).unwrap();
		let tokens = lexer::lex(&src).unwrap();
		// let program = parser::Parser::new(&tokens).parse().unwrap();
		println!("{:#?}", tokens);
		// println!("{:#?}", program);
	} else {
		println!("fatal: no input file found");
		std::process::exit(1);
	}
}
