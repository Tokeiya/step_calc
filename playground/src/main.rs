use std::io::{Read, Write};
use std::ops::Index;

use anyhow::{Context, Result};

use parser::infix::parser::parse;
use regex::Regex;
use syntax::dot_writer::write_dot;

mod html_writer;

fn main() {
	const INPUT: &str = r"hello
world";

	let re = Regex::new(r"(?m)&.*$").unwrap();
	for caps in re.captures_iter(INPUT) {
		println!("First word: {}, Second word: {}", &caps[1], &caps[2]);
	}

	// let tree = parse("1+2*{3+4}").unwrap().0;
	// let mut cursor = std::io::Cursor::<Vec<u8>>::default();
	//
	// write_dot(&mut cursor, &tree).unwrap();
	// let str = String::from_utf8(cursor.into_inner()).unwrap();
	// let str = html_writer::generate_svg(&str).unwrap();
	//
	// let reg = regex::Regex::new(r"(?m)^(.*)$").unwrap();
	//
	// println!("{}", &str);
	//
	// let cap = reg.captures(&str).unwrap();
	//
	// for elem in cap.iter() {
	// 	println!("{:?}", elem)
	// }
}

fn bar() {
	let re = regex::Regex::new(r"(?m)^(.*)$").unwrap();
	let text = "First line\nSecond line\nThird line";

	for cap in re.captures_iter(text) {
		if let Some(matched) = cap.get(1) {
			// キャプチャグループ1にアクセス
			println!("Captured line: {}", matched.as_str());
		}
	}
}
