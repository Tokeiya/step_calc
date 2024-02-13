mod parser_sample;

use combine::Parser;
use parser_sample::*;
use std::io::prelude::*;
use std::io::{self, Cursor, Write};

pub enum NumericValue {
	Integer(i32),
	Real(f64),
}

impl From<i32> for NumericValue {
	fn from(value: i32) -> Self {
		NumericValue::Integer(value)
	}
}

impl From<f64> for NumericValue {
	fn from(value: f64) -> Self {
		NumericValue::Real(value)
	}
}

fn main() {
	let ans = expr().parse("10+-2*3/{4+5}");
	println!("{:?}", ans);
}
