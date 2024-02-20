use std::env;
use std::fs::File;
use std::io::Cursor;
use std::io::{BufWriter, Write};
use syntax::arithmetic_expression::ArithmeticExpression;
use syntax::*;

fn main() {
	println!("{}", env::current_dir().unwrap().display());

	let expr = parser::infix::parser::parse("20+40+30*20-{400/4}")
		.unwrap()
		.0;

	println!(
		"{}",
		parser::infix::formatter::strict_infix_expression(&expr)
	);

	let mut tmp = expr.step_calc();

	while tmp.1 {
		println!(
			"{}",
			parser::infix::formatter::strict_infix_expression(&tmp.0)
		);

		tmp = tmp.0.step_calc();
	}
}
