use std::env;
use std::fs::File;
use std::io::Cursor;
use std::io::{BufRead, Read, Write};
use std::ops::Index;
use std::process::Command;

use anyhow::Context;

use parser::infix::parser::parse;
use syntax::arithmetic_expression::ArithmeticExpression;
use syntax::dot_writer::write_dot;

mod html_writer;

fn main() {
	let tree = parse("{1+2*3}/{{4-5}*{{6+7}/2}}").unwrap().0.simplify();
	let tree = tree.simplify();

	let mut cursor = Cursor::<Vec<u8>>::default();

	_ = write_dot(&mut cursor, &tree).unwrap();

	let str = String::from_utf8(cursor.into_inner()).unwrap();
	println!("{}", &str)
}

fn write_samples() {
	let current_dir = env::current_dir().unwrap();
	println!("The current directory is {}", current_dir.display());
	let tree = parse("{1+2*3}/{{4-5}*{{6+7}/2}}").unwrap().0.simplify();
	{
		let mut file = File::create("./playground/test_artifacts/sample.dot").unwrap();
		write_dot(&mut file, &tree).unwrap();
	}

	{
		let mut proc = Command::new("dot")
			.args(&[
				r"-Tsvg",
				r".\playground\test_artifacts\sample.dot",
				r"-o",
				r".\playground\test_artifacts\sample.svg",
			])
			.output()
			.unwrap();

		println!("{:?}", proc)
	}
}
