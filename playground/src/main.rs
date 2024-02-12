use std::io::prelude::*;
use std::io::{self, Cursor, Write};

fn write_hello<W: Write>(writer: &mut W) -> io::Result<()> {
	write!(writer, "hello")
}

fn main() {
	let mut str = Cursor::new(Vec::<u8>::new());

	write_hello(&mut str);

	println!("{}", String::from_utf8(str.into_inner()).unwrap())
}
