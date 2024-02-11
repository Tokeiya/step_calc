use crate::arithmetic_expression::ArithmeticExpression;
use crate::binary_operation::{BinaryOperation, Operation};
use crate::bracket::Bracket;
use crate::expression::Expression;
use crate::number::Number as NumberExpr;
use crate::number_value::{NumberResult, NumberValue};
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

fn write_header<W: Write>(writer: &mut W) -> io::Result<()> {
	writeln!(writer, "digraph arithmetic_tree{{")?;
	writeln!(writer, r#"   node [fontname = "Cascadia Code Regular"];"#)?;
	writeln!(writer)
}
pub fn write_dot<W: Write>(writer: &mut W) -> io::Result<()> {
	todo!()
}
