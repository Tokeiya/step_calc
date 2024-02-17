use combine::parser::char::{self as chr, char, digit};
use combine::{chainl1, choice, many1, parser, token, ParseError, Parser, Stream};

use syntax::arithmetic_expression::ArithmeticExpression;
use syntax::binary_operation::{BinaryOperation, Operation};
use syntax::bracket::Bracket;
use syntax::expression::Expression;
use syntax::number::Number;
use syntax::number_value::NumberValue;

fn trim<Input, O>(parser: impl Parser<Input, Output = O>) -> impl Parser<Input, Output = O>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(chr::spaces(), parser, chr::spaces()).map(|(_, expr, _)| expr)
}

fn expr_<Input>() -> impl Parser<Input, Output = Expression>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	let unsigned = trim(many1(digit()).map(|v: String| {
		Number::from(NumberValue::from(v.parse::<i32>().unwrap())).to_expression()
	}));

	let signed = trim(
		char('-')
			.with(many1(digit()).map(|v: String| v))
			.map(|v: String| {
				Number::from(NumberValue::from(v.parse::<i32>().unwrap() * -1)).to_expression()
			}),
	);

	let digit = choice((signed, unsigned));

	let bracket_expr = trim(
		(char::<Input>('{'), expr(), char('}')).map(|(_, e, _)| Bracket::from(e).to_expression()),
	);

	let primary = choice((digit, bracket_expr));

	let op = trim(choice((token::<Input>('*'), token('/'))).map(|c| {
		if c == '*' {
			Operation::Mul
		} else {
			Operation::Div
		}
	}));

	let multitive_chain = trim(op.map(|o: Operation| {
		move |l: Expression, r: Expression| BinaryOperation::new(l, r, o).to_expression()
	}));

	let op = trim(choice((token::<Input>('+'), token('-'))).map(|c| {
		if c == '+' {
			Operation::Add
		} else {
			Operation::Sub
		}
	}));

	let additive_chain = trim(op.map(|o| {
		move |l: Expression, r: Expression| BinaryOperation::new(l, r, o).to_expression()
	}));

	let multitive = trim(chainl1(primary, multitive_chain));

	let additive = trim(chainl1(multitive, additive_chain));

	additive
}

// parser! {
// 	pub fn expr[Input]()(Input)->Expression
// 	where [Input:Stream<Token = char>]{
// 		expr_()
// 	}
// }

#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct expr<Input>
where
	<Input as ::combine::stream::StreamOnce>::Error: ::combine::error::ParseError<
		<Input as ::combine::stream::StreamOnce>::Token,
		<Input as ::combine::stream::StreamOnce>::Range,
		<Input as ::combine::stream::StreamOnce>::Position,
	>,
	Input: ::combine::stream::Stream,
	Input: Stream<Token = char>,
{
	__marker: ::combine::lib::marker::PhantomData<fn(Input) -> Expression>,
}
#[allow(non_shorthand_field_patterns)]
impl<Input> ::combine::Parser<Input> for expr<Input>
where
	<Input as ::combine::stream::StreamOnce>::Error: ::combine::error::ParseError<
		<Input as ::combine::stream::StreamOnce>::Token,
		<Input as ::combine::stream::StreamOnce>::Range,
		<Input as ::combine::stream::StreamOnce>::Position,
	>,
	Input: ::combine::stream::Stream,
	Input: Stream<Token = char>,
{
	type Output = Expression;
	type PartialState = ();

	#[inline]
	fn parse_partial(
		&mut self,
		input: &mut Input,
		state: &mut Self::PartialState,
	) -> ::combine::error::ParseResult<Self::Output, <Input as ::combine::StreamOnce>::Error> {
		self.parse_mode(::combine::parser::PartialMode::default(), input, state)
	}
	#[inline]
	fn parse_first(
		&mut self,
		input: &mut Input,
		state: &mut Self::PartialState,
	) -> ::combine::error::ParseResult<Self::Output, <Input as ::combine::StreamOnce>::Error> {
		self.parse_mode(::combine::parser::FirstMode, input, state)
	}
	#[inline]
	fn parse_mode_impl<M>(
		&mut self,
		mode: M,
		input: &mut Input,
		state: &mut Self::PartialState,
	) -> ::combine::error::ParseResult<Expression, <Input as ::combine::stream::StreamOnce>::Error>
	where
		M: ::combine::parser::ParseMode,
	{
		let expr { .. } = *self;
		{
			let _ = state;
			let mut state = Default::default();
			let state = &mut state;
			{ expr_() }.parse_mode(mode, input, state)
		}
	}

	#[inline]
	fn add_error(
		&mut self,
		errors: &mut ::combine::error::Tracked<<Input as ::combine::stream::StreamOnce>::Error>,
	) {
		let expr { .. } = *self;
		let mut parser = { expr_() };
		{
			let _: &mut dyn ::combine::Parser<Input, Output = Expression, PartialState = _> =
				&mut parser;
		}
		parser.add_error(errors)
	}

	fn add_committed_expected_error(
		&mut self,
		errors: &mut ::combine::error::Tracked<<Input as ::combine::stream::StreamOnce>::Error>,
	) {
		let expr { .. } = *self;
		let mut parser = { expr_() };
		{
			let _: &mut dyn ::combine::Parser<Input, Output = Expression, PartialState = _> =
				&mut parser;
		}
		parser.add_committed_expected_error(errors)
	}
}
#[inline]
pub fn expr<Input>() -> expr<Input>
where
	<Input as ::combine::stream::StreamOnce>::Error: ::combine::error::ParseError<
		<Input as ::combine::stream::StreamOnce>::Token,
		<Input as ::combine::stream::StreamOnce>::Range,
		<Input as ::combine::stream::StreamOnce>::Position,
	>,
	Input: ::combine::stream::Stream,
	Input: Stream<Token = char>,
{
	expr {
		__marker: ::combine::lib::marker::PhantomData,
	}
}

#[cfg(test)]
mod tests {
	use super::expr;
	use combine::Parser;
	use std::io::Cursor;
	use syntax::arithmetic_expression::ArithmeticExpression;
	use syntax::dot_writer::write_dot;

	use super::*;
	use syntax::expression_manipulator::simplify;

	#[test]
	fn hoge() {
		let mut cursor = Cursor::<Vec<u8>>::default();

		let expr = expr()
			.parse("{ 30       *            {     1+2}-25}/{10+20+15       }")
			.unwrap();
		let expr = simplify(&expr.0);

		write_dot(&mut cursor, &expr).unwrap();

		let str = String::from_utf8(cursor.into_inner()).unwrap();
		println!("{str}");
		println!("{:?}", expr.calc().unwrap())
	}
}
