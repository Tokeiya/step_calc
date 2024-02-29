use regex::Regex;
use std::fs::File;
use std::io::Cursor;

mod html_writer;

fn main() {
	const INPUT: &str = r"10 20    30 40  + -        *";

	let mut cursor = Cursor::<Vec<char>>::new(INPUT.chars().collect());

	let reg = Regex::new(r"\s+").unwrap();
	let array: Vec<_> = reg.split(INPUT).collect();

	println!("{}", array.len());

	for elem in array.iter() {
		println!("{}", elem)
	}
}
