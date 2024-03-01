use std::collections::VecDeque;
use std::ops::Index;

use combine::stream::Range;
use parser::rpn::tokenizer::{tokenize, Token};
use syntax::binary_operation::Operation;

mod html_writer;

fn print(token: &Token) {
	match token {
		Token::Number(num) => println!("{:?}", num),
		Token::Operator(op) => match op {
			Operation::Add => println!("+"),
			Operation::Sub => println!("-"),
			Operation::Mul => println!("*"),
			Operation::Div => println!("/"),
		},
	}
}

fn main() {
	let (mut v, s) = tokenize("   10 20 300 + /");

	while let Some(t) = v.pop() {
		print(&t)
	}
}
