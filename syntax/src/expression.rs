use crate::arithmetic_expression::ArithmeticExpression;
use crate::binary_operation::BinaryOperation;
use crate::bracket::Bracket;
use crate::number::Number;
use crate::number_value::NumberResult;

pub enum Expression {
	Number(Number),
	Bracket(Bracket),
	BinaryOperation(BinaryOperation),
}

impl From<Bracket> for Expression {
	fn from(value: Bracket) -> Self {
		Expression::Bracket(value)
	}
}

impl From<Number> for Expression {
	fn from(value: Number) -> Self {
		Expression::Number(value)
	}
}

impl From<BinaryOperation> for Expression {
	fn from(value: BinaryOperation) -> Self {
		Expression::BinaryOperation(value)
	}
}

impl Clone for Expression {
	fn clone(&self) -> Self {
		match self {
			Expression::Number(x) => Expression::Number(x.clone()),
			Expression::Bracket(x) => Expression::Bracket(x.clone()),
			Expression::BinaryOperation(x) => Expression::BinaryOperation(x.clone()),
		}
	}
}

impl ArithmeticExpression for Expression {
	fn calc(&self) -> NumberResult {
		match self {
			Expression::Number(x) => x.calc(),
			Expression::Bracket(x) => x.calc(),
			Expression::BinaryOperation(x) => x.calc(),
		}
	}
	
	fn to_expression(self) -> Expression {
		self
	}
	
	fn simplify(&self) -> Expression {
		match self {
			Expression::Number(num) => num.simplify(),
			Expression::Bracket(bra) => bra.simplify(),
			Expression::BinaryOperation(bin) => bin.simplify(),
		}
	}
	
	fn step_calc(&self) -> (Expression, bool) {
		match self {
			Expression::Number(num) => num.step_calc(),
			Expression::Bracket(bracket) => bracket.step_calc(),
			Expression::BinaryOperation(bin) => bin.step_calc(),
		}
	}
}

#[cfg(any(feature = "test_active", test))]
pub mod helper {
	use crate::binary_operation::BinaryOperation;
	use crate::bracket::Bracket;
	use crate::expression::Expression;
	use crate::number::Number as NumberExpr;
	use crate::number_value::NumberValue;
	
	impl Expression {
		pub fn extract_as_number(&self) -> &NumberExpr {
			match self {
				Expression::Number(number) => number,
				_ => panic!("Cannot extract as number"),
			}
		}
		
		pub fn extract_as_binary_operation(&self) -> &BinaryOperation {
			match self {
				Expression::BinaryOperation(op) => op,
				_ => unreachable!(),
			}
		}
		
		pub fn extract_as_bracket(&self) -> &Bracket {
			match self {
				Expression::Bracket(x) => x,
				_ => unreachable!(),
			}
		}
	}
	
	fn create_number() -> NumberExpr {
		NumberExpr::from(NumberValue::from(100))
	}
	
	fn create_binary_operation_fixture() -> BinaryOperation {
		BinaryOperation::new(
			NumberExpr::from(NumberValue::from(100)),
			NumberExpr::from(NumberValue::from(200)),
			crate::binary_operation::Operation::Add,
		)
	}
	
	fn create_bracket_fixture() -> Bracket {
		Bracket::from(Expression::Number(NumberExpr::from(NumberValue::from(100))))
	}
	
	#[test]
	fn extract_number() {
		let fixture = Expression::from(create_number());
		fixture.extract_as_number().number().eq_i32(&100);
	}
	
	#[test]
	#[should_panic]
	fn invalid_extract_number_bracket() {
		let fixture = Expression::from(create_bracket_fixture());
		fixture.extract_as_number();
	}
	
	#[test]
	#[should_panic]
	fn invalid_extract_number_binary() {
		let fixture = Expression::from(create_binary_operation_fixture());
		fixture.extract_as_number();
	}
	
	#[test]
	fn extract_binary_operation() {
		let fixture = Expression::from(create_binary_operation_fixture());
		fixture.extract_as_binary_operation();
	}
	
	#[test]
	#[should_panic]
	fn invalid_extract_binary_operation_number() {
		let fixture = Expression::from(create_number());
		fixture.extract_as_binary_operation();
	}
	
	#[test]
	#[should_panic]
	fn invalid_extract_binary_operation_bracket() {
		let fixture = Expression::from(create_bracket_fixture());
		fixture.extract_as_binary_operation();
	}
	
	#[test]
	fn extract_bracket() {
		let fixture = Expression::from(create_bracket_fixture());
		fixture.extract_as_bracket();
	}
	
	#[test]
	#[should_panic]
	fn invalid_extract_bracket_number() {
		let fixture = Expression::from(create_number());
		fixture.extract_as_bracket();
	}
	
	#[test]
	#[should_panic]
	fn invalid_extract_bracket_binary() {
		let fixture = Expression::from(create_binary_operation_fixture());
		fixture.extract_as_bracket();
	}
}

#[cfg(test)]
mod tests {
	use crate::binary_operation::Operation;
	use crate::number::Number as NumberExpr;
	use crate::number_value::NumberValue;
	
	use super::*;
	
	#[test]
	fn step_calc() {
		let fixture = NumberExpr::from(NumberValue::from(200)).to_expression();
		let fixture = fixture.step_calc();
		
		assert!(!fixture.1);
		fixture.0.extract_as_number().number().eq_i32(&200);
		
		let fixture = Bracket::from(NumberExpr::from(NumberValue::from(42)).to_expression()).to_expression();
		let fixture = fixture.step_calc();
		assert!(fixture.1);
		fixture.0.extract_as_number().number().eq_i32(&42);
		
		let fixture = BinaryOperation::new(
			NumberExpr::from(NumberValue::from(42)),
			NumberExpr::from(NumberValue::from(100)),
			Operation::Add,
		).to_expression();
		let fixture = fixture.step_calc();
		assert!(fixture.1);
		fixture.0.extract_as_number().number().eq_i32(&142);
	}
	
	#[test]
	fn simplify() {
		//Number.
		let fixture = Expression::from(Number::from(NumberValue::from(200)));
		let fixture = fixture.simplify();
		
		fixture.extract_as_number().number().eq_i32(&200);
		
		//Bracket.
		let fixture = Expression::from(Bracket::from(
			Number::from(NumberValue::from(400)).to_expression(),
		));
		let fixture = fixture.simplify();
		
		fixture.extract_as_number().number().eq_i32(&400);
		
		//BinOp
		let fixture = Expression::from(BinaryOperation::new(
			Number::from(NumberValue::from(10)),
			Number::from(NumberValue::from(30)),
			Operation::Add,
		));
		
		let fixture = fixture.simplify();
		let fixture = fixture.extract_as_binary_operation();
		
		fixture.left().extract_as_number().number().eq_i32(&10);
		fixture.right().extract_as_number().number().eq_i32(&30);
		
		assert!(matches!(fixture.operation(), Operation::Add));
	}
	
	#[test]
	fn from_number() {
		let fixture = Expression::from(Number::from(NumberValue::from(300)));
		fixture.extract_as_number().number().eq_i32(&300);
	}
	
	#[test]
	fn from_binary_operation() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		
		let bin = BinaryOperation::new(left, right, Operation::Add);
		let fixture = Expression::from(bin);
		let fixture = fixture.extract_as_binary_operation();
		
		fixture.left().extract_as_number().number().eq_i32(&200);
		fixture.right().extract_as_number().number().eq_i32(&300);
		matches!(fixture.operation(), &Operation::Add);
	}
	
	#[test]
	fn from_bracket() {
		let fixture = Expression::from(Bracket::from(Expression::Number(Number::from(
			NumberValue::from(300),
		))));
		fixture.extract_as_bracket().expression().extract_as_number().number().eq_i32(&300);
	}
	
	#[test]
	fn to_expression() {
		let fixture = Expression::from(Number::from(NumberValue::from(300)));
		let expr = fixture.to_expression();
		expr.extract_as_number().number().eq_i32(&300);
		
		let fixture = Expression::from(Bracket::from(Expression::Number(Number::from(
			NumberValue::from(300),
		))));
		let expr = fixture.to_expression();
		expr.extract_as_bracket().expression().extract_as_number().number().eq_i32(&300);
		
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		
		let bin = BinaryOperation::new(left, right, Operation::Add);
		let fixture = Expression::from(bin);
		let expr = fixture.to_expression();
		
		let fixture = expr.extract_as_binary_operation();
		
		fixture.left().extract_as_number().number().eq_i32(&200);
		fixture.right().extract_as_number().number().eq_i32(&300);
		matches!(fixture.operation(), &Operation::Add);
	}
	
	#[test]
	fn calc() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		
		let bin = BinaryOperation::new(left, right, Operation::Add);
		let fixture = Expression::from(bin);
		fixture.calc().unwrap().eq_i32(&500);
	}
	
	#[test]
	fn clone() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));
		
		let bin = BinaryOperation::new(left, right, Operation::Mul);
		let mut fixture = Expression::from(bin);
		
		let cloned = fixture.clone();
		
		fixture = Expression::from(Number::from(NumberValue::from(200)));
		
		fixture.extract_as_number().number().eq_i32(&200);
		
		let fixture = cloned.extract_as_binary_operation();
		
		fixture.left().extract_as_number().number().eq_i32(&200);
		fixture.right().extract_as_number().number().eq_i32(&300);
		matches!(fixture.operation(), &Operation::Mul);
	}
}
