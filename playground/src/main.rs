use regex::Regex;
use std::fs::File;
use std::io::Cursor;

mod html_writer;

fn main() {
	let str = "0123456789";
	let slice = &str[..0];
	println!("{slice}");

	for i in (0..10).rev() {}
}
