use std::collections::{HashSet, VecDeque};
use std::iter::Peekable;
use std::str::CharIndices;

use once_cell::sync::Lazy;

use syntax::binary_operation::Operation;
use syntax::number_value::NumberValue;

type CharIterator<'a> = Peekable<CharIndices<'a>>;

#[derive(Debug)]
pub enum Token {
	Number(NumberValue),
	Operator(Operation),
}

static NUM: Lazy<HashSet<char>> = Lazy::new(|| HashSet::from_iter("0123456789".chars()));
static OPS: Lazy<HashSet<char>> = Lazy::new(|| HashSet::from_iter("+-*/".chars()));

fn skip_whitespace(iterator: &mut CharIterator) {
	while let Some((_, c)) = iterator.peek() {
		if c.is_whitespace() {
			iterator.next();
		} else {
			break;
		}
	}
}

fn get_number(iterator: &mut CharIterator) -> Token {
	let mut buff = String::new();

	while let Some((_, c)) = iterator.peek() {
		if NUM.contains(&c) {
			buff.push(*c);
		} else {
			break;
		}

		iterator.next();
	}
	Token::Number(NumberValue::from(buff.parse::<i32>().unwrap()))
}

fn get_operator(iterator: &mut CharIterator) -> Token {
	match iterator.next().unwrap().1 {
		'+' => Token::Operator(Operation::Add),
		'-' => Token::Operator(Operation::Sub),
		'*' => Token::Operator(Operation::Mul),
		'/' => Token::Operator(Operation::Div),
		_ => unreachable!(),
	}
}

fn try_get_token(iterator: &mut CharIterator) -> Option<Token> {
	if let Some((_, c)) = iterator.peek() {
		if OPS.contains(c) {
			Some(get_operator(iterator))
		} else if NUM.contains(c) {
			Some(get_number(iterator))
		} else {
			None
		}
	} else {
		None
	}
}

fn single_tokenize(scr: &str) -> (Option<Token>, &str) {
	let mut ite = scr.char_indices().peekable();

	skip_whitespace(&mut ite);
	let token = try_get_token(&mut ite);

	return if token.is_some() {
		if let Some((idx, _)) = ite.peek() {
			(token, &scr[*idx..])
		} else {
			(token, &scr[scr.len()..])
		}
	} else {
		(token, scr)
	};
}

pub fn tokenize(scr: &str) -> (VecDeque<Token>, &str) {
	let mut remainder = scr;
	let mut ret = VecDeque::<Token>::default();
	loop {
		let (token, rem) = single_tokenize(remainder);

		if let Some(t) = token {
			ret.push_front(t);
			remainder = rem;
		} else {
			break;
		}
	}

	(ret, remainder)
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
	fn skip_whitespace_test() {
		let mut iterator = "    2     ".char_indices().peekable();

		for _ in 0..10 {
			skip_whitespace(&mut iterator);
			assert_eq!(iterator.peek().unwrap().1, '2');
		}
	}

	#[test]
	fn get_number_test() {
		let mut iterator = "20   ".char_indices().peekable();
		let act = get_number(&mut iterator);

		act.assert_i32(&20);
		let (idx, c) = iterator.peek().unwrap();
		assert_eq!(idx, &2);
		assert_eq!(c, &' ');
	}

	#[test]
	fn get_operator_test() {
		let mut iterator = "+-*/".char_indices().peekable();

		let act = get_operator(&mut iterator);
		act.assert_operator(&Operation::Add);

		let act = get_operator(&mut iterator);
		act.assert_operator(&Operation::Sub);

		let act = get_operator(&mut iterator);
		act.assert_operator(&Operation::Mul);

		let act = get_operator(&mut iterator);
		act.assert_operator(&Operation::Div);

		assert!(iterator.peek().is_none())
	}

	#[test]
	fn single_tokenize_test() {
		let (token, remainder) = single_tokenize("      \t    10\t20       +       ");
		token.unwrap().assert_i32(&10);
		assert_eq!(remainder, "\t20       +       ");

		let (token, remainder) = single_tokenize(remainder);
		token.unwrap().assert_i32(&20);
		assert_eq!(remainder, "       +       ");

		let (token, remainder) = single_tokenize(remainder);
		token.unwrap().assert_operator(&Operation::Add);
		assert_eq!(remainder, "       ");

		for _ in 0..10 {
			let (token, remainder) = single_tokenize(remainder);
			assert!(token.is_none());
			assert_eq!(remainder, "       ");
		}

		let (token, remainder) = single_tokenize(" hoge ");
		assert!(token.is_none());
		assert_eq!(" hoge ", remainder);

		for _ in 0..10 {
			let (token, remainder) = single_tokenize(remainder);
			assert!(token.is_none());
			assert_eq!(" hoge ", remainder);
		}
	}

	#[test]
	fn tokenize_test() {
		let (vec, rem) = tokenize("10 20 30 / +");
		assert_eq!(vec.len(), 5);

		vec[4].assert_i32(&10);
		vec[3].assert_i32(&20);
		vec[2].assert_i32(&30);

		vec[1].assert_operator(&Operation::Div);
		vec[0].assert_operator(&Operation::Add);

		assert_eq!(rem, "");

		let (_vdc, rem) = tokenize("10 20 30 / + hoge");
		assert_eq!(vec.len(), 5);

		vec[4].assert_i32(&10);
		vec[3].assert_i32(&20);
		vec[2].assert_i32(&30);

		vec[1].assert_operator(&Operation::Div);
		vec[0].assert_operator(&Operation::Add);

		assert_eq!(rem, " hoge");
	}
}
