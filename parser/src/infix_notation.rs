use combine::parser::char::{char, digit};
use combine::{chainl1, choice, many1, parser, token, ParseError, Parser, Stream};

use syntax::arithmetic_expression::ArithmeticExpression;
use syntax::binary_operation::{BinaryOperation, Operation};
use syntax::bracket::Bracket;
use syntax::expression::Expression;
use syntax::number::Number;
use syntax::number_value::NumberValue;

fn strict_expression(expr: &Expression, buffer: &mut String) {
	match expr {
		Expression::Number(num) => strict_number(num, buffer),
		Expression::Bracket(bracket) => strict_bracket(bracket, buffer),
		Expression::BinaryOperation(bin_op) => strict_binary_op(bin_op, buffer),
	}
}

fn strict_number(number: &Number, buffer: &mut String) {
	match number.number() {
		NumberValue::Integer(int) => buffer.push_str(&format!(" {int}").to_string()),
	}
}

fn strict_binary_op(binary_operation: &BinaryOperation, buffer: &mut String) {
	buffer.push(' ');
	buffer.push('(');

	strict_expression(binary_operation.left(), buffer);
	buffer.push(' ');

	buffer.push(match binary_operation.operation() {
		Operation::Add => '+',
		Operation::Sub => '-',
		Operation::Mul => '*',
		Operation::Div => '/',
	});

	buffer.push(' ');
	strict_expression(binary_operation.right(), buffer);
	buffer.push(')');
}

fn strict_bracket(bracket: &Bracket, buffer: &mut String) {
	buffer.push(' ');
	buffer.push('{');

	strict_expression(bracket.expression(), buffer);

	buffer.push(' ');
	buffer.push('}');
}

pub fn strict_infix_expression(expr: &Expression) -> String {
	let mut buff = String::default();
	strict_expression(&(expr.clone().to_expression()), &mut buff);
	buff
}

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
	use std::io::Cursor;

	use combine::Parser;

	use syntax::arithmetic_expression::ArithmeticExpression;
	use syntax::dot_writer::write_dot;

	use super::*;
	use syntax::expression_manipulator::simplify;

	#[test]
	fn hoge() {
		let mut cursor = Cursor::<Vec<u8>>::default();

		let expr = expr().parse("{30*{1+2}-25}/{10+20+15}").unwrap();
		let expr = simplify(&expr.0);

		write_dot(&mut cursor, &expr).unwrap();

		let str = String::from_utf8(cursor.into_inner()).unwrap();
		println!("{str}");
		println!("{:?}", expr.calc().unwrap())
	}

	#[test]
	fn piyo() {
		let expr = expr().parse("{30*{1+2}-25}/{10+20+15}").unwrap().0;
		let ret = strict_infix_expression(&expr);
		println!("{ret}");
	}
}
