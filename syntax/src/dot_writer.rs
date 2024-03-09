use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::io::prelude::*;

use crate::arithmetic_expression::ArithmeticExpression;
use crate::binary_operation::{BinaryOperation, Operation};
use crate::bracket::Bracket;
use crate::expression::Expression;
use crate::id_dispatcher::IdDispatcher;
use crate::number::Number as NumberExpr;
use crate::number_value::NumberValue;

use super::id_dispatcher::IdDispatcherError;

// digraph sample{
//      node [fontname = "Cascadia Code Regular"];
//
//      box[label="NumberValue",shape = "box"];
//      hexagon[label="Operator",shape= "hexagon",fontname="Consolas"];
//      house[label="Bracket",shape="house"]
//
//      box->hexagon
// }

pub enum WriterError {
	IoError(io::Error),
	IdDispatcherError(IdDispatcherError),
}

impl Debug for WriterError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let msg = match self {
			WriterError::IoError(ioe) => format!("IoError:{:?}", ioe),
			WriterError::IdDispatcherError(x) => format!("IdError:{:?}", x),
		};

		writeln!(f, "{}", msg)
	}
}

impl Display for WriterError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Debug::fmt(self, f)
	}
}

impl std::error::Error for WriterError {}

type WriterResult = Result<(), WriterError>;

trait ToWriteError {
	fn map(self) -> WriterError;
}

impl ToWriteError for io::Error {
	fn map(self) -> WriterError {
		WriterError::IoError(self)
	}
}

impl ToWriteError for IdDispatcherError {
	fn map(self) -> WriterError {
		WriterError::IdDispatcherError(self)
	}
}

fn write_header<W: Write>(writer: &mut W) -> WriterResult {
	writeln!(writer, "digraph arithmetic_tree{{").map_err(|x| x.map())?;
	writeln!(writer, r#"   node [fontname = "Cascadia Code Regular"];"#).map_err(|x| x.map())?;
	writeln!(writer).map_err(|x| x.map())
}

fn write_footer<W: Write>(writer: &mut W) -> WriterResult {
	writeln!(writer, "}}").map_err(|x| x.map())
}

fn write_expression<W: Write>(
	writer: &mut W,
	dispatcher: &mut IdDispatcher,
	expression: &Expression,
) -> WriterResult {
	match expression {
		Expression::Number(num) => write_number(writer, dispatcher, &num),
		Expression::Bracket(bracket) => write_bracket(writer, dispatcher, &bracket),
		Expression::BinaryOperation(bin) => write_binary_operation(writer, dispatcher, &bin),
	}
}

fn write_direction<W: Write>(writer: &mut W, dispatcher: &IdDispatcher) -> WriterResult {
	if let Ok(p) = dispatcher.parent() {
		let id = dispatcher.current().map_err(|x| x.map())?;
		writeln!(writer, "\t{} -> {}", p, id).map_err(|x| x.map())?;
		Ok(())
	} else {
		Ok(())
	}
}

fn write_number<W: Write>(
	writer: &mut W,
	dispatcher: &mut IdDispatcher,
	number: &NumberExpr,
) -> WriterResult {
	let id = dispatcher.get().map_err(|x| x.map())?;
	let NumberValue::Integer(num) = number.number();

	writeln!(writer, "\t{} [label=\"{}\",shape=\"box\"]", id, num).map_err(|x| x.map())?;
	write_direction(writer, dispatcher)?;

	_ = dispatcher.pop().map_err(|x| x.map());
	Ok(())
}

fn write_bracket<W: Write>(
	writer: &mut W,
	dispatcher: &mut IdDispatcher,
	bracket: &Bracket,
) -> WriterResult {
	let id = dispatcher.get().map_err(|err| err.map())?;

	writeln!(writer, r#"	{} [label="{{...}}",shape = "house"]"#, id).map_err(|err| err.map())?;

	write_expression(writer, dispatcher, bracket.expression())?;

	write_direction(writer, dispatcher)?;
	_ = dispatcher.pop().map_err(|err| err.map())?;
	Ok(())
}

fn write_binary_operation<W: Write>(
	writer: &mut W,
	dispatcher: &mut IdDispatcher,
	binary_operation: &BinaryOperation,
) -> WriterResult {
	let id = dispatcher.get().map_err(|err| err.map())?;

	let op = match binary_operation.operation() {
		Operation::Add => "+",
		Operation::Sub => "-",
		Operation::Mul => "*",
		Operation::Div => "/",
	};

	writeln!(writer, r#"	{} [label="{}",shape = "hexagon"]"#, id, op).map_err(|err| err.map())?;
	write_direction(writer, dispatcher)?;

	write_expression(writer, dispatcher, binary_operation.left())?;
	write_expression(writer, dispatcher, binary_operation.right())?;

	dispatcher.pop().map_err(|err| err.map())?;

	Ok(())
}

pub fn write_dot<W: Write, E: ArithmeticExpression>(
	writer: &mut W,
	expression: &E,
) -> WriterResult {
	write_header(writer)?;
	let mut dispatcher = IdDispatcher::new();
	let expr = expression.clone().to_expression();

	write_expression(writer, &mut dispatcher, &expr)?;

	write_footer(writer)?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use std::io::Cursor;
	
	use super::*;
	
	#[test]
	fn write_header() {
		let mut cursor = Cursor::<Vec<u8>>::default();
		super::write_header(&mut cursor).unwrap();

		let actual = String::from_utf8(cursor.into_inner()).unwrap();

		assert_eq!(
			actual,
			r#"digraph arithmetic_tree{
   node [fontname = "Cascadia Code Regular"];

"#
		);
	}

	#[test]
	fn write_footer() {
		let mut cursor = Cursor::<Vec<u8>>::default();
		super::write_footer(&mut cursor).unwrap();

		let actual = String::from_utf8(cursor.into_inner()).unwrap();
		assert_eq!(actual, "}\n");
	}

	#[test]
	fn number() {
		let mut cursor = Cursor::<Vec<u8>>::default();
		let num = NumberExpr::from(NumberValue::from(42));
		let mut dispatcher = IdDispatcher::new();

		_ = write_number(&mut cursor, &mut dispatcher, &num);
		let actual = String::from_utf8(cursor.into_inner()).unwrap();

		assert_eq!("\t1 [label=\"42\",shape=\"box\"]\n", actual);
	}

	#[test]
	fn bracket() {
		let mut cursor = Cursor::<Vec<u8>>::default();
		let mut dispatcher = IdDispatcher::new();

		let num = NumberExpr::from(NumberValue::from(42));
		let brackert = Bracket::from(num.to_expression());

		_ = write_bracket(&mut cursor, &mut dispatcher, &brackert).unwrap();
		let actual = String::from_utf8(cursor.into_inner()).unwrap();

		assert_eq!(
			r#"	1 [label="{...}",shape = "house"]
	2 [label="42",shape="box"]
	1 -> 2
"#,
			actual
		);
	}

	#[test]
	fn binary_op() {
		let mut cursor = Cursor::<Vec<u8>>::default();
		let mut dispatcher = IdDispatcher::new();

		let left = NumberExpr::from(NumberValue::from(42));
		let right = NumberExpr::from(NumberValue::from(100));

		let bin = BinaryOperation::new(left, right, Operation::Sub);

		write_binary_operation(&mut cursor, &mut dispatcher, &bin).unwrap();

		let act = String::from_utf8(cursor.into_inner()).unwrap();

		assert_eq!(
			r#"	1 [label="-",shape = "hexagon"]
	2 [label="42",shape="box"]
	1 -> 2
	3 [label="100",shape="box"]
	1 -> 3
"#,
			act
		);
	}

	#[test]
	fn dot() {
		let mut cursor = Cursor::<Vec<u8>>::default();

		let left = NumberExpr::from(NumberValue::from(42));
		let right = NumberExpr::from(NumberValue::from(100));

		let bin = BinaryOperation::new(left, right, Operation::Add);

		let left = bin;
		let right = BinaryOperation::new(
			NumberExpr::from(NumberValue::from(2)),
			NumberExpr::from(NumberValue::from(3)),
			Operation::Mul,
		);

		let bin = BinaryOperation::new(Bracket::from(left.to_expression()), right, Operation::Div);

		write_dot(&mut cursor, &bin).unwrap();

		let act = String::from_utf8(cursor.into_inner()).unwrap();

		assert_eq!(
			act,
			r#"digraph arithmetic_tree{
   node [fontname = "Cascadia Code Regular"];

	1 [label="/",shape = "hexagon"]
	2 [label="{...}",shape = "house"]
	3 [label="+",shape = "hexagon"]
	2 -> 3
	4 [label="42",shape="box"]
	3 -> 4
	5 [label="100",shape="box"]
	3 -> 5
	1 -> 2
	6 [label="*",shape = "hexagon"]
	1 -> 6
	7 [label="2",shape="box"]
	6 -> 7
	8 [label="3",shape="box"]
	6 -> 8
}
"#
		);
	}
}
