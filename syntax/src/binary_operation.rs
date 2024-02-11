use crate::arithmetic_expression::ArithmeticExpression;
use crate::expression::Expression;
use crate::number_value::NumberValue;

pub enum Operation {
	Add,
	Sub,
	Mul,
	Div,
}

pub struct BinaryOperation {
	left: Box<Expression>,
	right: Box<Expression>,
	operation: Operation,
}

impl BinaryOperation {
	pub fn new(
		left: impl ArithmeticExpression,
		right: impl ArithmeticExpression,
		operation: Operation,
	) -> Self {
		Self {
			left: Box::new(left),
			right: Box::new(Expression::from(right)),
			operation,
		}
	}

	pub fn left(&self) -> &Expression {
		&self.left
	}

	pub fn right(&self) -> &Expression {
		&self.right
	}

	pub fn operation(&self) -> &Operation {
		&self.operation
	}
}

impl Clone for BinaryOperation {
	fn clone(&self) -> Self {
		todo!()
	}
}

impl ArithmeticExpression for BinaryOperation {
	fn calc(&self) -> NumberValue {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::BinaryOperation;
	use crate::arithmetic_expression::ArithmeticExpression;
	use crate::binary_operation::Operation;
	use crate::number::Number;
	use crate::number_value::NumberValue;

	#[test]
	fn new() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Add);

		fixture.left().extract_as_number().number().eq_i32(&200);
		fixture.right().extract_as_number().number().eq_i32(&300);
		matches!(fixture.operation(), &Operation::Add);
	}

	#[test]
	fn add() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Add);
		fixture.calc().eq_i32(&500);
	}

	#[test]
	fn mul() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Mul);
		fixture.calc().eq_i32(&60000);
	}

	#[test]
	fn sub() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Sub);
		fixture.calc().eq_i32(&-100);
	}

	#[test]
	fn div() {
		let left = Number::from(NumberValue::from(600));
		let right = Number::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Div);
		fixture.calc().eq_i32(&2);
	}

	#[test]
	fn left() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Add);
		fixture.left().extract_as_number().number().eq_i32(&200);
	}

	#[test]
	fn right() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Add);
		fixture.right().extract_as_number().number().eq_i32(&300);
	}

	#[test]
	fn operation() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Add);
		matches!(fixture.operation(), &Operation::Add);

		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Sub);
		matches!(fixture.operation(), &Operation::Sub);

		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Mul);
		matches!(fixture.operation(), &Operation::Mul);

		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Div);
		matches!(fixture.operation(), &Operation::Div);
	}

	#[test]
	fn calc() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Add);

		fixture.calc().eq_i32(&500);
	}

	#[test]
	fn clone() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Add);

		let act = fixture.clone();
		act.left().extract_as_number().number().eq_i32(&200);
		act.right().extract_as_number().number().eq_i32(&300);
		matches!(act.operation(), &Operation::Add);
	}
}
