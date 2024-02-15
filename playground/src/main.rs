#![feature(lazy_cell)]

mod parser_sample;

use combine::Parser;
use parser_sample::*;
use std::collections::VecDeque;
use std::io::prelude::*;
use std::io::{self, Cursor, Write};

use std::sync::{LazyLock, OnceLock};

static FOO: LazyLock<usize> = LazyLock::new(|| 42usize);

fn parser_sample() {
	let ans = expr().parse("10+-2*3/{4+5}");
	println!("{:?}", ans);
}

fn main() {}
