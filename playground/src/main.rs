#![feature(lazy_cell)]

use std::io::prelude::*;
use std::io::Write;
use std::sync::LazyLock;

use combine::Parser;

use parser_sample::*;

mod foo;
mod parser_sample;

static FOO: LazyLock<usize> = LazyLock::new(|| 42usize);

fn parser_sample() {
	let ans = expr().parse("10+-2*3/{4+5}");
	println!("{:?}", ans);
}

fn main() {
	let mut buff = Vec::<String>::new();
	buff.push("hello".to_string());

	let mut str = String::new();
}
