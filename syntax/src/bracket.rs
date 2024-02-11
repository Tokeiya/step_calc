use crate::arithmetic_expression::ArithmeticExpression;
use crate::expression::Expression;
use crate::number_value::NumberValue;

pub struct Bracket(Box<Expression>);

impl From<Expression> for Bracket {
	fn from(value: Expression) -> Self {
		Bracket(Box::new(value))
	}
}

impl Clone for Bracket {
	fn clone(&self) -> Self {
		Bracket(self.0.clone())
	}
}

impl ArithmeticExpression for Bracket {
	fn calc(&self) -> NumberValue {
		self.0.calc()
	}
}

impl Bracket {
	pub fn expression(&self) -> &Expression {
		&self.0
	}
}
#[cfg(test)]
mod tests {
	use super::Bracket;
	use crate::arithmetic_expression::ArithmeticExpression;
	use crate::binary_operation::{BinaryOperation, Operation};
	use crate::expression::Expression;
	use crate::number::Number;
	use crate::number_value::NumberValue;

	#[test]
	fn from() {
		let fixture = Bracket::from(Expression::Number(Number::from(NumberValue::from(300))));
		fixture.0.extract_as_number().number().eq_i32(&200)
	}

	#[test]
	fn calc() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));

		let bin = BinaryOperation::new(left, right, Operation::Mul);
		let bracket = Bracket::from(Expression::BinaryOperation(bin));

		let act = bracket.0.calc();
		act.eq_i32(&600000);
	}

	#[test]
	fn clone() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));

		let bin = BinaryOperation::new(left, right, Operation::Mul);
		let mut bracket = Bracket::from(Expression::BinaryOperation(bin));

		let cloned = bracket.clone();

		bracket = Bracket::from(Expression::Number(Number::from(NumberValue::from(200))));

		bracket.0.extract_as_number().number().eq_i32(&200);

		let fixture = cloned.0.extract_as_binary_operation();

		fixture.left().extract_as_number().number().eq_i32(&200);
		fixture.right().extract_as_number().number().eq_i32(&300);
		matches!(fixture.operation(), &Operation::Mul);
	}

	#[test]
	fn expression() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));

		let bin = BinaryOperation::new(left, right, Operation::Mul);
		let bracket = Bracket::from(Expression::BinaryOperation(bin));

		let fixture = bracket.expression().extract_as_binary_operation();
		fixture.right().extract_as_number().number().eq_i32(&200);
		fixture.left().extract_as_number().number().eq_i32(&300);
		matches!(fixture.operation(), &Operation::Mul);
	}
}