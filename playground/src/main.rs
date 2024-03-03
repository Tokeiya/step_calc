use parser::rpn::parser::Token;
use syntax::binary_operation::Operation;

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

fn main() {}
