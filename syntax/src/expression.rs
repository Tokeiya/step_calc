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
}

#[cfg(test)]
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
		let fixture = Expression::Number(create_number());
		fixture.extract_as_number().number().eq_i32(&100);
	}

	#[test]
	#[should_panic]
	fn invalid_extract_number_bracket() {
		let fixture = Expression::Bracket(create_bracket_fixture());
		fixture.extract_as_number();
	}

	#[test]
	#[should_panic]
	fn invalid_extract_number_binary() {
		let fixture = Expression::BinaryOperation(create_binary_operation_fixture());
		fixture.extract_as_number();
	}

	#[test]
	fn extract_binary_operation() {
		let fixture = Expression::BinaryOperation(create_binary_operation_fixture());
		fixture.extract_as_binary_operation();
	}

	#[test]
	#[should_panic]
	fn invalid_extract_binary_operation_number() {
		let fixture = Expression::Number(create_number());
		fixture.extract_as_binary_operation();
	}

	#[test]
	#[should_panic]
	fn invalid_extract_binary_operation_bracket() {
		let fixture = Expression::Bracket(create_bracket_fixture());
		fixture.extract_as_binary_operation();
	}

	#[test]
	fn extract_bracket() {
		let fixture = Expression::Bracket(create_bracket_fixture());
		fixture.extract_as_bracket();
	}

	#[test]
	#[should_panic]
	fn invalid_extract_bracket_number() {
		let fixture = Expression::Number(create_number());
		fixture.extract_as_bracket();
	}

	#[test]
	#[should_panic]
	fn invalid_extract_bracket_binary() {
		let fixture = Expression::BinaryOperation(create_binary_operation_fixture());
		fixture.extract_as_bracket();
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::binary_operation::Operation;
	use crate::number::Number;
	use crate::number_value::NumberValue;

	#[test]
	fn to_expression() {
		let fixture = Expression::Number(Number::from(NumberValue::from(300)));
		let expr = fixture.to_expression();
		expr.extract_as_number().number().eq_i32(&300);

		let fixture = Expression::Bracket(Bracket::from(Expression::Number(Number::from(
			NumberValue::from(300),
		))));
		let expr = fixture.to_expression();
		expr.extract_as_bracket()
			.expression()
			.extract_as_number()
			.number()
			.eq_i32(&300);

		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));

		let bin = BinaryOperation::new(left, right, Operation::Add);
		let fixture = Expression::BinaryOperation(bin);
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
		let fixture = Expression::BinaryOperation(bin);
		fixture.calc().unwrap().eq_i32(&500);
	}

	#[test]
	fn clone() {
		let left = Number::from(NumberValue::from(200));
		let right = Number::from(NumberValue::from(300));

		let bin = BinaryOperation::new(left, right, Operation::Mul);
		let mut fixture = Expression::BinaryOperation(bin);

		let cloned = fixture.clone();

		fixture = Expression::Number(Number::from(NumberValue::from(200)));

		fixture.extract_as_number().number().eq_i32(&200);

		let fixture = cloned.extract_as_binary_operation();

		fixture.left().extract_as_number().number().eq_i32(&200);
		fixture.right().extract_as_number().number().eq_i32(&300);
		matches!(fixture.operation(), &Operation::Mul);
	}
}
