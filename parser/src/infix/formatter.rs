use syntax::arithmetic_expression::ArithmeticExpression;
use syntax::binary_operation::{BinaryOperation, Operation};
use syntax::bracket::Bracket;
use syntax::expression::Expression;
use syntax::number::Number;
use syntax::number_value::NumberValue;

fn write_operator(operator: &Operation, buffer: &mut String) {
	buffer.push(' ');

	match operator {
		Operation::Add => buffer.push('+'),
		Operation::Sub => buffer.push('-'),
		Operation::Mul => buffer.push('*'),
		Operation::Div => buffer.push('/'),
	}

	buffer.push(' ');
}

fn minimal_expression(expr: &Expression, parent: &Option<&Operation>, buffer: &mut String) {
	match expr {
		Expression::Number(num) => minimal_number(num, buffer),
		Expression::Bracket(bracket) => minimal_bracket(bracket, parent, buffer),
		Expression::BinaryOperation(bin) => minimal_binary_op(bin, parent, buffer),
	}
}

fn is_additive(operator: &Operation) -> bool {
	match operator {
		Operation::Add => true,
		Operation::Sub => true,
		Operation::Mul => false,
		Operation::Div => false,
	}
}

fn require(expr: &BinaryOperation, parent: &Option<&Operation>) -> bool {
	match parent {
		None => false,
		Some(p) => {
			if let Operation::Div = p {
				true
			} else if is_additive(p) {
				false
			} else {
				is_additive(expr.operation())
			}
		}
	}
}

fn minimal_binary_op(expr: &BinaryOperation, parent: &Option<&Operation>, buffer: &mut String) {
	if require(expr, parent) {
		buffer.push('{');
	}

	minimal_expression(expr.left(), &Some(expr.operation()), buffer);

	write_operator(expr.operation(), buffer);

	minimal_expression(expr.right(), &Some(expr.operation()), buffer);

	if require(expr, parent) {
		buffer.push('}');
	}
}

fn minimal_bracket(expr: &Bracket, parent: &Option<&Operation>, buffer: &mut String) {
	minimal_expression(expr.expression(), parent, buffer)
}

fn minimal_number(expr: &Number, buffer: &mut String) {
	match expr.number() {
		NumberValue::Integer(int) => buffer.push_str(&format!("{}", int)),
	}
}

pub fn minimal_infix_notation(expr: &Expression) -> String {
	let mut buff = String::default();
	minimal_expression(expr, &None, &mut buff);
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
	write_operator(binary_operation.operation(), buffer);
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
	use combine::Parser;

	use syntax::binary_operation::{BinaryOperation, Operation};
	use syntax::number::Number as NumExpr;
	use syntax::number_value::NumberValue;

	use crate::infix::formatter::{minimal_infix_notation, strict_infix_expression};
	use crate::infix::parser::get_parser;

	use super::require;

	#[test]
	fn strict() {
		let expr = get_parser().parse("{30*{1+2}-25}/{10+20+15}").unwrap().0;
		let ret = strict_infix_expression(&expr);
		assert_eq!(ret, "{{{{30 * {{1 + 2}}} - 25}} / {{{10 + 20} + 15}}}");
	}

	#[test]
	fn require_test() {
		fn make_fixture(operator: Operation) -> BinaryOperation {
			BinaryOperation::new(
				NumExpr::from(NumberValue::from(42)),
				NumExpr::from(NumberValue::from(114514)),
				operator,
			)
		}

		let bin = make_fixture(Operation::Add);
		assert!(!require(&bin, &Some(&Operation::Add)));
		assert!(!require(&bin, &Some(&Operation::Sub)));
		assert!(!require(&bin, &None));
		assert!(require(&bin, &Some(&Operation::Mul)));
		assert!(require(&bin, &Some(&Operation::Div)));

		let bin = make_fixture(Operation::Sub);
		assert!(!require(&bin, &Some(&Operation::Add)));
		assert!(!require(&bin, &Some(&Operation::Sub)));
		assert!(!require(&bin, &None));
		assert!(require(&bin, &Some(&Operation::Mul)));
		assert!(require(&bin, &Some(&Operation::Div)));

		let bin = make_fixture(Operation::Mul);
		assert!(!require(&bin, &Some(&Operation::Add)));
		assert!(!require(&bin, &Some(&Operation::Sub)));
		assert!(!require(&bin, &None));
		assert!(!require(&bin, &Some(&Operation::Mul)));
		assert!(!require(&bin, &Some(&Operation::Div)));

		let bin = make_fixture(Operation::Div);
		assert!(!require(&bin, &Some(&Operation::Add)));
		assert!(!require(&bin, &Some(&Operation::Sub)));
		assert!(!require(&bin, &None));
		assert!(!require(&bin, &Some(&Operation::Mul)));
		assert!(!require(&bin, &Some(&Operation::Div)));
	}

	#[test]
	fn minimal() {
		let expr = get_parser().parse("{{30*{1+2}-25}/{10+20+15}}").unwrap().0;
		let ret = minimal_infix_notation(&expr);
		assert_eq!(ret, "{30 * {1 + 2} - 25} / {10 + 20 + 15}");
	}
}
