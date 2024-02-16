use combine::parser::char::{char, digit};
use combine::{chainl1, choice, many1, parser, token, ParseError, Parser, Stream};
use syntax::arithmetic_expression::ArithmeticExpression;
use syntax::binary_operation::{BinaryOperation, Operation};
use syntax::bracket::Bracket;
use syntax::expression::Expression;
use syntax::number::Number;
use syntax::number_value::{NumberResult, NumberValue};
fn expr_<Input>() -> impl Parser<Input, Output = Expression>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	let unsigned = many1(digit()).map(|v: String| {
		Number::from(NumberValue::from(v.parse::<i32>().unwrap())).to_expression()
	});

	let signed = char('-')
		.with(many1(digit()).map(|v: String| v))
		.map(|v: String| {
			Number::from(NumberValue::from(v.parse::<i32>().unwrap() * -1)).to_expression()
		});

	let digit = choice((signed, unsigned));

	let bracket_expr =
		(char::<Input>('{'), expr(), char('}')).map(|(_, e, _)| Bracket::from(e).to_expression());

	let primary = choice((digit, bracket_expr));

	let op = choice((token::<Input>('*'), token('/'))).map(|c| {
		if c == '*' {
			Operation::Mul
		} else {
			Operation::Div
		}
	});

	let multitive_chain = op.map(|o: Operation| {
		move |l: Expression, r: Expression| BinaryOperation::new(l, r, o).to_expression()
	});

	let op = choice((token::<Input>('+'), token('-'))).map(|c| {
		if c == '+' {
			Operation::Add
		} else {
			Operation::Sub
		}
	});

	let additive_chain = op
		.map(|o| move |l: Expression, r: Expression| BinaryOperation::new(l, r, o).to_expression());

	let multitive = chainl1(primary, multitive_chain);

	let additive = chainl1(multitive, additive_chain);

	additive
}

parser! {
	pub fn expr[Input]()(Input)->Expression
	where [Input:Stream<Token = char>]{
		expr_()
	}
}

#[cfg(test)]
mod tests {
	use super::expr;
	use combine::Parser;
	use std::fmt::Write;
	use std::io::Cursor;
	use syntax::arithmetic_expression::ArithmeticExpression;
	use syntax::dot_writer::{write_dot, WriterError};

	#[test]
	fn hoge() {
		let mut cursor = Cursor::<Vec<u8>>::default();

		let expr = expr().parse("{10+20}*30").unwrap();

		write_dot(&mut cursor, &expr.0).unwrap();

		let str = String::from_utf8(cursor.into_inner()).unwrap();
		println!("{str}");
		println!("{:?}", expr.0.calc().unwrap())
	}
}
