use std::fs::File;
use parser::rpn::parser::Token;
use syntax::binary_operation::Operation;
use rpn_html_writer::write_html;

#[allow(dead_code)]
mod infix_html_writer;
mod rpn_html_writer;
mod test_helper;

#[cfg(test)]
mod test_writer;

#[allow(dead_code)]
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
	let file=File::create("rpn_first_light.html").unwrap();
	
	write_html("16 8 4 2 - * +",&file).unwrap()
	
}
