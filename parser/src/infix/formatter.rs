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
		NumberValue::Integer(int) => buffer.push_str(&format!("{int}").to_string()),
	}
}

fn strict_binary_op(binary_operation: &BinaryOperation, buffer: &mut String) {
	buffer.push('{');

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
	use crate::infix::formatter::strict_infix_expression;
	use crate::infix::parser::get_parser;
	use combine::Parser;

	#[test]
	fn piyo() {
		let expr = get_parser().parse("{30*{1+2}-25}/{10+20+15}").unwrap().0;
		let ret = strict_infix_expression(&expr);
		println!("{ret}");
	}
}
