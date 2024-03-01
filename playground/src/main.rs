use std::ops::Index;

use combine::stream::Range;

mod html_writer;

fn main() {
	let str = "1";
	let a = &str[1..];
	println!("{a}")
}
