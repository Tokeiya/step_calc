use combine::stream::Range;
use std::ops::Index;

mod html_writer;

fn main() {
	const SAMPLE: &str = "abcdefghijklmnopqrstuvwxyz";
	let mut a = SAMPLE.char_indices().enumerate();

	let a = &SAMPLE[0..20];
	for elem in a {
		println!("{:?}", elem)
	}
}
