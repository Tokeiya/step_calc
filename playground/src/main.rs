use combine::stream::Range;
use parser::rpn::char_cursor::CharCursor;
use regex::Regex;
use std::fs::File;
use std::io::Cursor;

mod html_writer;

fn main() {
	const SAMPLE: &str = "abcdefghijklmnopqrstuvwxyz";
	println!("{}", SAMPLE.len());
}
