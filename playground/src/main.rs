use combine::stream::Range;
use std::ops::Index;

mod html_writer;

fn main() {
	let str = "1";
	let a = &str[1..];
	println!("{a}")
}
