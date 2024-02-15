use super::id_dispatcher::{IdDispatcherError, IdResult};
use crate::arithmetic_expression::ArithmeticExpression;
use crate::binary_operation::{BinaryOperation, Operation};
use crate::bracket::Bracket;
use crate::expression::Expression;
use crate::id_dispatcher::IdDispatcher;
use crate::number::Number as NumberExpr;
use crate::number_value::{NumberResult, NumberValue};
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::io::prelude::*;

// digraph sample{
//      node [fontname = "Cascadia Code Regular"];
//
//      box[label="NumberValue",shape = "box"];
//      hexagon[label="Operator",shape= "hexagon",fontname="Consolas"];
//      house[label="Bracket",shape="house"]
//
//      box->hexagon
// }

enum WriterError {
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
	expression: Expression,
) -> WriterResult {
	match expression {
		Expression::Number(num) => write_number(writer, dispatcher, &num),
		Expression::Bracket(bracket) => write_bracket(writer, dispatcher, &bracket),
		Expression::BinaryOperation(bin) => write_binary_operation(writer, dispatcher, &bin),
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

	if let Ok(p) = dispatcher.parent() {
		writeln!(writer, "\t{} -> {}", p, id).map_err(|x| x.map())?
	}

	_ = dispatcher.pop().map_err(|x| x.map());
	Ok(())
}

fn write_bracket<W: Write>(
	writer: &mut W,
	dispatcher: &mut IdDispatcher,
	bracket: &Bracket,
) -> WriterResult {
	todo!()
}

fn write_binary_operation<W: Write>(
	writer: &mut W,
	dispatcher: &mut IdDispatcher,
	binary_operation: &BinaryOperation,
) -> WriterResult {
	todo!()
}

pub fn write_dot<W: Write, E: ArithmeticExpression>(
	writer: &mut W,
	expression: &E,
) -> WriterResult {
	write_header(writer)?;

	todo!()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::arithmetic_expression::ArithmeticExpression;
	use crate::binary_operation::{BinaryOperation, Operation};
	use crate::bracket::Bracket;
	use crate::expression::Expression;
	use std::io::{self, Cursor, Write};

	type IoResult = io::Result<()>;

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

		write_number()
		
		todo!()
	}
}
