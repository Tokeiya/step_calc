use crate::arithmetic_expression::ArithmeticExpression;
use crate::binary_operation::{BinaryOperation, Operation};
use crate::bracket::Bracket;
use crate::expression::Expression;
use crate::id_dispatcher::IdDispatcher;
use crate::number::Number as NumberExpr;
use crate::number_value::{NumberResult, NumberValue};
use std::io;
use std::io::prelude::*;

type IoResult = io::Result<()>;

// digraph sample{
//      node [fontname = "Cascadia Code Regular"];
//
//      box[label="NumberValue",shape = "box"];
//      hexagon[label="Operator",shape= "hexagon",fontname="Consolas"];
//      house[label="Bracket",shape="house"]
//
//      box->hexagon
// }

fn write_header<W: Write>(writer: &mut W) -> IoResult {
	writeln!(writer, "digraph arithmetic_tree{{")?;
	writeln!(writer, r#"   node [fontname = "Cascadia Code Regular"];"#)?;
	writeln!(writer)
}

fn write_footer<W: Write>(writer: &mut W) -> IoResult {
	writeln!(writer, "}}")
}

fn write_expression<W: Write>(
	writer: &mut W,
	dispatcher: &mut IdDispatcher,
	expression: Expression,
) -> IoResult {
	todo!()
}

fn write_number<W: Write>(
	writer: &mut W,
	dispatcher: &mut IdDispatcher,
	number: &NumberExpr,
) -> IoResult {
	todo!()
}

fn write_bracket<W: Write>(
	writer: &mut W,
	dispatcher: &mut IdDispatcher,
	bracket: &Bracket,
) -> IoResult {
	todo!()
}

fn write_binary_operation<W: Write>(
	writer: &mut W,
	dispatcher: &mut IdDispatcher,
	binary_operation: &BinaryOperation,
) {
	todo!()
}

pub fn write_dot<W: Write, E: ArithmeticExpression>(
	writer: &mut W,
	expression: &E,
) -> io::Result<()> {
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
}
