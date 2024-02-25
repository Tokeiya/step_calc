use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::ops::Index;

use parser::infix::parser::parse;
use regex::Regex;
use syntax::arithmetic_expression::ArithmeticExpression;
use syntax::dot_writer::write_dot;

use std::env;
use std::io::Cursor;
use std::process::{Command, Stdio};

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
