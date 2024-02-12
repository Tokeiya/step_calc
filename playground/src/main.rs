use std::io::prelude::*;
use std::io::{self, Cursor, Write};
use syntax::expression::Expression;

struct Value(i32);

impl From<i32> for Value {
	fn from(value: i32) -> Self {
		Value(value)
	}
}

fn main() {}
