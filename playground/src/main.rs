use parser::infix::parser::parse;
use std::env;

use parser::infix::formatter::{minimal_infix_notation, strict_infix_expression};
use syntax::arithmetic_expression::ArithmeticExpression;
use syntax::dot_writer::{write_dot, WriterError};

fn main() {
	//	let expr = parse("{10+30*{40+2}}/{1+2/3}").unwrap().0;
}
