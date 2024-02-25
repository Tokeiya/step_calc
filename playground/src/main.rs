use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::ops::Index;

use parser::infix::parser::parse;
use regex::Regex;
use syntax::arithmetic_expression::ArithmeticExpression;
use syntax::dot_writer::write_dot;

mod html_writer;

fn main() {
	check();
}

fn check() {
	let tree = parse("{{{10+20*3}/{{4-5}*{{6+7}/2}}}}").unwrap().0;
	println!("{:?}", tree.calc());
	let minimal = parser::infix::formatter::minimal_infix_notation(&tree);
	println!("{}", &minimal);
}

fn write_sample_dot() {
	let tree = parse("{1+2*3}/{{4-5}*{{6+7}/2}}").unwrap().0.simplify();
	let mut cursor = std::io::Cursor::<Vec<u8>>::default();

	syntax::dot_writer::write_dot(&mut cursor, &tree).unwrap();
	println!("{}", String::from_utf8(cursor.into_inner()).unwrap());

	let a = parser::infix::formatter::minimal_infix_notation(&tree);
	println!("{a}");

	let tree = parse("{1 + 2 * 3} / {4 - 5} * {6 + 7} / 2")
		.unwrap()
		.0
		.simplify();

	let mut cursor = std::io::Cursor::<Vec<u8>>::default();

	syntax::dot_writer::write_dot(&mut cursor, &tree).unwrap();
	println!("{}", String::from_utf8(cursor.into_inner()).unwrap());

	let a = parser::infix::formatter::minimal_infix_notation(&tree);
	println!("{a}");
}
