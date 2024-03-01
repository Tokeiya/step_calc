use once_cell::sync::Lazy;
use regex::{Error as RegError, Regex};
use syntax::binary_operation::Operation;
use syntax::number_value::NumberValue;

pub enum Token {
	Number(NumberValue),
	Operator(Operation),
}

fn single_tokenize(scr: &str) -> (Token, bool, &str) {
	todo!()
}

pub fn tokenize(scr: &str) -> (Vec<Token>, &str) {
	todo!()
}

#[cfg(test)]
pub mod helper {
	use syntax::binary_operation::Operation;

	use super::Token;

	fn get_order(value: &Operation) -> usize {
		match value {
			Operation::Add => 1,
			Operation::Sub => 2,
			Operation::Mul => 3,
			Operation::Div => 4,
		}
	}

	impl Token {
		pub fn assert_i32(&self, expectd: &i32) {
			if let Token::Number(i) = self {
				i.eq_i32(expectd)
			} else {
				unreachable!()
			}
		}

		pub fn assert_operator(&self, expected: &Operation) {
			if let Token::Operator(op) = self {
				assert_eq!(get_order(op), get_order(expected))
			} else {
				unreachable!()
			}
		}
	}
}

#[cfg(test)]
pub mod tests {
	use super::*;

	#[test]
	fn tokenize_test() {
		let (vec, rem) = tokenize("10 20 30 / +");
		assert_eq!(vec.len(), 5);

		vec[0].assert_i32(&10);
		vec[1].assert_i32(&20);
		vec[2].assert_i32(&30);

		vec[3].assert_operator(&Operation::Div);
		vec[3].assert_operator(&Operation::Add);

		assert_eq!(rem, "");

		let (vdc, rem) = tokenize("10 20 30 / + hoge");
		assert_eq!(vec.len(), 5);

		vec[0].assert_i32(&10);
		vec[1].assert_i32(&20);
		vec[2].assert_i32(&30);

		vec[3].assert_operator(&Operation::Div);
		vec[3].assert_operator(&Operation::Add);

		assert_eq!(rem, " hoge");
	}
}
