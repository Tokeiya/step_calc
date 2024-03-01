use std::collections::VecDeque;
use std::ops::Index;

use combine::stream::Range;
use parser::rpn::parser::{tokenize, Token};
use syntax::binary_operation::Operation;

mod infix_html_writer;
mod rpn_html_writer;

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
	let mut vec = Vec::<i32>::default();

	vec.push(0);
	vec.push(1);

	println!("{:?}", vec.pop());
	println!("{:?}", vec.pop());
}
