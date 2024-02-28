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

use crate::html_writer::{extract_svg_element, generate_svg, write_header};
use html_writer::write_infix_html;

fn main() {
	let str = "hello world".to_string();

	for elem in str.chars().enumerate() {}
}

// fn write_samples() {
// 	let current_dir = env::current_dir().unwrap();
// 	println!("The current directory is {}", current_dir.display());
// 	let tree = parse("{1+2*3}/{{4-5}*{{6+7}/2}}").unwrap().0.simplify();
// 	{
// 		let mut file = File::create("./playground/test_artifacts/sample.dot").unwrap();
// 		write_dot(&mut file, &tree).unwrap();
// 	}
//
// 	{
// 		let mut proc = Command::new("dot")
// 			.args(&[
// 				r"-Tsvg",
// 				r".\playground\test_artifacts\sample.dot",
// 				r"-o",
// 				r".\playground\test_artifacts\sample.svg",
// 			])
// 			.output()
// 			.unwrap();
//
// 		println!("{:?}", proc)
// 	}
// }
