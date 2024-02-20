use crate::arithmetic_expression::ArithmeticExpression;
use crate::expression::Expression;
use crate::number::Number;
use crate::number_value::NumberResult;

#[derive(Clone)]
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
			left: Box::new(left.to_expression()),
			right: Box::new(right.to_expression()),
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
		BinaryOperation::new(
			self.left.as_ref().clone(),
			self.right.as_ref().clone(),
			self.operation.clone(),
		)
	}
}

impl ArithmeticExpression for BinaryOperation {
	fn calc(&self) -> NumberResult {
		let left = self.left.calc()?;
		let right = self.right.calc()?;

		match self.operation {
			Operation::Add => left + right,
			Operation::Sub => left - right,
			Operation::Mul => left * right,
			Operation::Div => left / right,
		}
	}

	fn to_expression(self) -> Expression {
		Expression::BinaryOperation(self)
	}

	fn simplify(&self) -> Expression {
		let left = self.left.simplify();
		let right = self.right.simplify();

		BinaryOperation::new(left, right, self.operation.clone()).to_expression()
	}

	fn step_calc(&self) -> (Expression, bool) {
		let tmp = self.left.step_calc();
		if tmp.1 {
			return (
				BinaryOperation::new(tmp.0, *self.right.clone(), self.operation.clone())
					.to_expression(),
				true,
			);
		}

		let tmp = self.right.step_calc();
		if tmp.1 {
			return (
				BinaryOperation::new(*self.left.clone(), tmp.0, self.operation.clone())
					.to_expression(),
				true,
			);
		}

		let tmp = self.calc().unwrap();

		(Number::from(tmp).to_expression(), true)
	}
}

#[cfg(test)]
mod tests {
	use super::BinaryOperation;
	use crate::arithmetic_expression::ArithmeticExpression;
	use crate::binary_operation::Operation;
	use crate::bracket::Bracket;
	use crate::number::Number as NumberExpr;
	use crate::number_value::NumberValue;

	#[test]
	fn step_calc() {
		let left = BinaryOperation::new(
			NumberExpr::from(NumberValue::from(2)),
			NumberExpr::from(NumberValue::from(3)),
			Operation::Mul,
		);
		let right = BinaryOperation::new(
			NumberExpr::from(NumberValue::from(4)),
			NumberExpr::from(NumberValue::from(2)),
			Operation::Sub,
		);

		let fixture = BinaryOperation::new(left, right, Operation::Mul);

		let fixture = fixture.step_calc();
		assert!(fixture.1);

		let fixture = fixture.0.extract_as_binary_operation();
		fixture.left.extract_as_number().number().eq_i32(&6);

		_ = fixture.right().extract_as_binary_operation();

		let fixture = fixture.step_calc();
		assert!(fixture.1);

		let fixture = fixture.0.extract_as_binary_operation();
		fixture.left.extract_as_number().number().eq_i32(&6);
		fixture.right.extract_as_number().number().eq_i32(&2);

		let fixture = fixture.step_calc();
		assert!(fixture.1);
		let fixture = fixture.0.extract_as_number();
		fixture.number().eq_i32(&12);

		let fixture = fixture.step_calc();
		assert!(!fixture.1);
		let fixture = fixture.0.extract_as_number();
		fixture.number().eq_i32(&12);
	}

	#[test]
	fn simplify() {
		let left = Bracket::from(NumberExpr::from(NumberValue::from(20)).to_expression());
		let right = NumberExpr::from(NumberValue::from(2));

		let bin = BinaryOperation::new(left.to_expression(), right.to_expression(), Operation::Mul);
		let act = bin.simplify();
		let act = act.extract_as_binary_operation();

		act.left().extract_as_number().number().eq_i32(&20);
	}

	#[test]
	fn new() {
		let left = NumberExpr::from(NumberValue::from(200));
		let right = NumberExpr::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Add);

		fixture.left().extract_as_number().number().eq_i32(&200);
		fixture.right().extract_as_number().number().eq_i32(&300);
		matches!(fixture.operation(), &Operation::Add);
	}

	#[test]
	fn add() {
		let left = NumberExpr::from(NumberValue::from(200));
		let right = NumberExpr::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Add);
		fixture.calc().unwrap().eq_i32(&500);
	}

	#[test]
	fn mul() {
		let left = NumberExpr::from(NumberValue::from(200));
		let right = NumberExpr::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Mul);
		fixture.calc().unwrap().eq_i32(&60000);
	}

	#[test]
	fn sub() {
		let left = NumberExpr::from(NumberValue::from(200));
		let right = NumberExpr::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Sub);
		fixture.calc().unwrap().eq_i32(&-100);
	}

	#[test]
	fn div() {
		let left = NumberExpr::from(NumberValue::from(600));
		let right = NumberExpr::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Div);
		fixture.calc().unwrap().eq_i32(&2);
	}

	#[test]
	fn to_expression() {
		let left = NumberExpr::from(NumberValue::from(200));
		let right = NumberExpr::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Add);
		let expr = fixture.to_expression();

		expr.extract_as_binary_operation()
			.left()
			.extract_as_number()
			.number()
			.eq_i32(&200);
		expr.extract_as_binary_operation()
			.right()
			.extract_as_number()
			.number()
			.eq_i32(&300);
		matches!(
			expr.extract_as_binary_operation().operation(),
			&Operation::Add
		);
	}

	#[test]
	fn left() {
		let left = NumberExpr::from(NumberValue::from(200));
		let right = NumberExpr::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Add);
		fixture.left().extract_as_number().number().eq_i32(&200);
	}

	#[test]
	fn right() {
		let left = NumberExpr::from(NumberValue::from(200));
		let right = NumberExpr::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Add);
		fixture.right().extract_as_number().number().eq_i32(&300);
	}

	#[test]
	fn operation() {
		let left = NumberExpr::from(NumberValue::from(200));
		let right = NumberExpr::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Add);
		matches!(fixture.operation(), &Operation::Add);

		let left = NumberExpr::from(NumberValue::from(200));
		let right = NumberExpr::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Sub);
		matches!(fixture.operation(), &Operation::Sub);

		let left = NumberExpr::from(NumberValue::from(200));
		let right = NumberExpr::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Mul);
		matches!(fixture.operation(), &Operation::Mul);

		let left = NumberExpr::from(NumberValue::from(200));
		let right = NumberExpr::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Div);
		matches!(fixture.operation(), &Operation::Div);
	}

	#[test]
	fn calc() {
		let left = NumberExpr::from(NumberValue::from(200));
		let right = NumberExpr::from(NumberValue::from(300));
		let fixture = BinaryOperation::new(left, right, Operation::Add);

		fixture.calc().unwrap().eq_i32(&500);
	}

	#[test]
	fn clone() {
		let left = NumberExpr::from(NumberValue::from(200));
		let right = NumberExpr::from(NumberValue::from(300));
		let mut fixture = BinaryOperation::new(left, right, Operation::Add);
		let act = fixture.clone();

		fixture.left = Box::new(NumberExpr::from(NumberValue::from(2)).to_expression());
		fixture.right = Box::new(NumberExpr::from(NumberValue::from(3)).to_expression());
		fixture.operation = Operation::Sub;

		fixture.calc().unwrap().eq_i32(&-1);

		act.left().extract_as_number().number().eq_i32(&200);
		act.right().extract_as_number().number().eq_i32(&300);
		matches!(act.operation(), &Operation::Add);
	}
}
