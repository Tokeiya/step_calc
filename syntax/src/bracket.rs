use crate::arithmetic_expression::ArithmeticExpression;
use crate::expression::Expression;
use crate::number_value::NumberResult;

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
	fn calc(&self) -> NumberResult {
		self.0.calc()
	}
	
	fn to_expression(self) -> Expression {
		Expression::from(self)
	}
	
	fn simplify(&self) -> Expression {
		self.0.simplify()
		//*self.0.clone()
	}
	
	fn step_calc(&self) -> (Expression, bool) {
		(*self.0.clone(), true)
	}
}

impl Bracket {
	pub fn expression(&self) -> &Expression {
		&self.0
	}
}

#[cfg(test)]
mod tests {
	use crate::arithmetic_expression::ArithmeticExpression;
	use crate::binary_operation::{BinaryOperation, Operation};
	use crate::expression::Expression;
	use crate::number::Number;
	use crate::number_value::NumberValue;
	
	use super::Bracket;
	
	#[test]
	fn step_calc() {
		let fixture = Bracket::from(Number::from(NumberValue::from(300)).to_expression());
		let fixture = fixture.step_calc();
		
		assert!(fixture.1);
		fixture.0.extract_as_number().number().eq_i32(&300);
	}
	
	#[test]
	fn simplify() {
		let fixture = Bracket::from(Number::from(NumberValue::from(300)).to_expression());
		let act = fixture.simplify();
		
		act.extract_as_number().number().eq_i32(&300);
		
		let fixture = Number::from(NumberValue::from(42));
		let fixture = Bracket::from(fixture.to_expression());
		let fixture = Bracket::from(fixture.to_expression());
		let fixture = Bracket::from(fixture.to_expression());
		let fixture = Bracket::from(fixture.to_expression());
		
		fixture.simplify().extract_as_number().number().eq_i32(&42);
	}
	
	#[test]
	fn from() {
		let fixture = Bracket::from(Expression::Number(Number::from(NumberValue::from(300))));
		fixture.0.extract_as_number().number().eq_i32(&300)
	}
	
	#[test]
	fn calc() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		
		let bin = BinaryOperation::new(left, right, Operation::Mul);
		let bracket = Bracket::from(Expression::BinaryOperation(bin));
		
		let act = bracket.calc();
		act.unwrap().eq_i32(&60_000);
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
		let expr = bracket.to_expression();
		
		let fixture = expr.extract_as_bracket().expression().extract_as_binary_operation();
		fixture.left().extract_as_number().number().eq_i32(&200);
		fixture.right().extract_as_number().number().eq_i32(&300);
		matches!(fixture.operation(), &Operation::Mul);
	}
}
