use syntax::arithmetic_expression::ArithmeticExpression;
use syntax::binary_operation::{BinaryOperation, Operation};
use syntax::bracket::Bracket;
use syntax::expression::Expression;
use syntax::number::Number;
use syntax::number_value::NumberValue;

fn write_operator(operator: &Operation, buffer: &mut String) {
	match operator {
		Operation::Add => buffer.push('+'),
		Operation::Sub => buffer.push('-'),
		Operation::Mul => buffer.push('*'),
		Operation::Div => buffer.push('/'),
	}
}

fn minimal_expression(expr: &Expression, parent: Option<Operation>, buffer: &mut String) {
	match expr {
		Expression::Number(num) => minimal_number(num, None, buffer),
		Expression::Bracket(bracket) => minimal_bracket(bracket, None, buffer),
		Expression::BinaryOperation(bin) => {
			minimal_binary_op(bin, Some(bin.operation().clone()), buffer);
		}
	}

	println!("expr:{}", buffer)
}

fn is_additive(operator: &Operation) -> bool {
	match operator {
		Operation::Add => true,
		Operation::Sub => true,
		Operation::Mul => false,
		Operation::Div => false,
	}
}

fn require(expr: &BinaryOperation, parent: &Option<Operation>) -> bool {
	match parent {
		None => false,
		Some(p) => {
			if is_additive(p) {
				false
			} else {
				if is_additive(expr.operation()) {
					true
				} else {
					false
				}
			}
		}
	}
}

fn minimal_binary_op(expr: &BinaryOperation, parent: Option<Operation>, buffer: &mut String) {
	let require = require(expr, &parent);

	if require {
		buffer.push('{');
	}

	minimal_expression(expr.left(), Some(expr.operation().clone()), buffer);
	write_operator(expr.operation(), buffer);
	minimal_expression(expr.right(), Some(expr.operation().clone()), buffer);

	if require {
		buffer.push('}');
	}

	println!("binary:{}", buffer)
}

fn minimal_bracket(expr: &Bracket, parent: Option<Operation>, buffer: &mut String) {
	match expr.expression() {
		Expression::Number(num) => minimal_number(num, None, buffer),
		Expression::Bracket(bracket) => minimal_bracket(bracket, None, buffer),
		Expression::BinaryOperation(bin) => {
			minimal_binary_op(bin, Some(bin.operation().clone()), buffer)
		}
	}

	println!("bracket:{}", buffer)
}

fn minimal_number(expr: &Number, parent: Option<Operation>, buffer: &mut String) {
	match expr.number() {
		NumberValue::Integer(num) => buffer.push_str(&format!("{}", num)),
	}

	println!("number:{}", buffer)
}

pub fn minimal_infix_notation(expr: &Expression) -> String {
	let mut buff = String::default();

	match expr {
		Expression::Number(num) => minimal_number(num, None, &mut buff),
		Expression::Bracket(bracket) => minimal_bracket(bracket, None, &mut buff),
		Expression::BinaryOperation(bin) => {
			minimal_binary_op(bin, Some(bin.operation().clone()), &mut buff)
		}
	};

	buff
}

fn strict_expression(expr: &Expression, buffer: &mut String) {
	match expr {
		Expression::Number(num) => strict_number(num, buffer),
		Expression::Bracket(bracket) => strict_bracket(bracket, buffer),
		Expression::BinaryOperation(bin_op) => strict_binary_op(bin_op, buffer),
	}
}

fn strict_number(number: &Number, buffer: &mut String) {
	match number.number() {
		NumberValue::Integer(int) => buffer.push_str(&format!("{int}").to_string()),
	}
}

fn strict_binary_op(binary_operation: &BinaryOperation, buffer: &mut String) {
	buffer.push('{');

	strict_expression(binary_operation.left(), buffer);

	buffer.push(' ');

	write_operator(binary_operation.operation(), buffer);

	buffer.push(' ');

	strict_expression(binary_operation.right(), buffer);
	buffer.push('}');
}

fn strict_bracket(bracket: &Bracket, buffer: &mut String) {
	buffer.push('{');

	strict_expression(bracket.expression(), buffer);

	buffer.push('}');
}

pub fn strict_infix_expression(expr: &Expression) -> String {
	let mut buff = String::default();
	strict_expression(&(expr.clone().to_expression()), &mut buff);
	buff
}

#[cfg(test)]
mod tests {
	use crate::infix::formatter::{minimal_infix_notation, strict_infix_expression};
	use crate::infix::parser::get_parser;
	use combine::Parser;
	use std::io::Cursor;
	use syntax::dot_writer::write_dot;

	#[test]
	fn strict() {
		let expr = get_parser().parse("{30*{1+2}-25}/{10+20+15}").unwrap().0;
		let ret = strict_infix_expression(&expr);
		assert_eq!(ret, "{{{{30 * {{1 + 2}}} - 25}} / {{{10 + 20} + 15}}}");
	}

	#[test]
	fn simple() {
		let expr = get_parser().parse("{{30*{1+2}-25}/{10+20+15}}").unwrap().0;

		let mut cursor = Cursor::<Vec<u8>>::default();
		write_dot(&mut cursor, &expr).unwrap();
		println!("{}", String::from_utf8(cursor.into_inner()).unwrap());

		let ret = minimal_infix_notation(&expr);

		println!("{}", ret);
		//todo!()
	}
}
