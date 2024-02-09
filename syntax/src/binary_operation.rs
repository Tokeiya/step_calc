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
	pub fn new(left: impl ArithmeticExpression, ritht: impl ArithmeticExpression, operation: Operation) -> Self {
		todo!()
	}

	pub fn left(&self) -> &Expression {
		todo!()
	}

	pub fn right(&self) -> &Expression {
		todo!()
	}

	pub fn operation(&self) -> &Operation {
		todo!()
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
mod tests {}
