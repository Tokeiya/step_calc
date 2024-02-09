use crate::arithmetic_expression::ArithmeticExpression;
use crate::binary_operation::BinaryOperation;
use crate::bracket::Bracket;
use crate::number::Number;
use crate::number_value::NumberValue;

pub enum Expression {
	Number(Number),
	Bracket(Bracket),
	BinaryOperation(BinaryOperation),
}

impl From<Bracket> for Expression {
	fn from(value: Bracket) -> Self {
		todo!()
	}
}

impl From<Number> for Expression {
	fn from(value: Number) -> Self {
		todo!()
	}
}

impl From<BinaryOperation> for Expression {
	fn from(value: BinaryOperation) -> Self {
		todo!()
	}
}

impl Clone for Expression {
	fn clone(&self) -> Self {
		todo!()
	}
}

impl ArithmeticExpression for Expression {
	fn calc(&self) -> NumberValue {
		todo!()
	}
}

#[cfg(test)]
pub mod helper {
	use crate::arithmetic_expression::ArithmeticExpression;
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

	#[test]
	fn extract_as_number() {
		let fixture = Expression::Number(NumberExpr::from(NumberValue::from(200)));
		let a = fixture.extract_as_number();
		a.number().eq_i32(&200);
	}

	#[test]
	#[should_panic]
	fn invalid_extract_as_number_a() {
		let fixture = Expression::Number(NumberExpr::from(NumberValue::from(200)));
		let tmp = fixture.extract_as_bracket();
	}

	#[test]
	#[should_panic]
	fn invalid_extract_as_number_b() {
		let fixture = Expression::Number(NumberExpr::from(NumberValue::from(200)));
		let tmp = fixture.extract_as_binary_operation();
	}

	fn create_binary_operation_fixture() -> BinaryOperation {
		BinaryOperation::new(
			NumberExpr::from(NumberValue::from(100)),
			NumberExpr::from(NumberValue::from(200)),
			crate::binary_operation::Operation::Add,
		)
	}

	#[test]
	fn extract_as_binary_operation() {
		let fixture = Expression::BinaryOperation(create_binary_operation_fixture());
		let a = fixture.extract_as_binary_operation();
		a.left().extract_as_number().number().eq_i32(&100);
		a.right().extract_as_number().number().eq_i32(&200);
	}

	#[test]
	#[should_panic]
	fn invalid_extract_as_binary_operation() {
		let fixture = Expression::BinaryOperation(create_binary_operation_fixture());
		let tmp = fixture.extract_as_bracket();
	}

	#[test]
	#[should_panic]
	fn invalid_extract_as_binary_operation_b() {
		let fixture = Expression::BinaryOperation(create_binary_operation_fixture());
		let tmp = fixture.extract_as_number();
	}

	fn create_bracket_fixture() -> Bracket {
		Bracket::from(Expression::Number(NumberExpr::from(NumberValue::from(100))))
	}
}
