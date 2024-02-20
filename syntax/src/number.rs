use crate::arithmetic_expression::ArithmeticExpression;
use crate::expression::Expression;
use crate::number_value::{NumberResult, NumberValue};

pub struct Number(NumberValue);
impl From<NumberValue> for Number {
	fn from(value: NumberValue) -> Self {
		Number(value)
	}
}

impl ArithmeticExpression for Number {
	fn calc(&self) -> NumberResult {
		Ok(self.0.clone())
	}

	fn to_expression(self) -> Expression {
		Expression::from(self)
	}

	fn simplify(&self) -> Expression {
		self.clone().to_expression()
	}
}

impl Number {
	pub fn number(&self) -> &NumberValue {
		&self.0
	}
}

impl Clone for Number {
	fn clone(&self) -> Self {
		Number::from(self.0.clone())
	}
}

#[cfg(test)]
mod tests {
	use crate::arithmetic_expression::ArithmeticExpression;
	use crate::number::Number;
	use crate::number_value::NumberValue;

	fn create_fixture(value: i32) -> Number {
		Number::from(NumberValue::from(value))
	}

	#[test]
	fn simplify() {
		let fixture = create_fixture(42);
		let act = fixture.simplify();

		act.extract_as_number().number().eq_i32(&42);
	}

	#[test]
	fn to_expression() {
		let fixture = create_fixture(42);
		let expr = fixture.to_expression();

		expr.extract_as_number().0.eq_i32(&42);
	}

	#[test]
	fn from_number_test() {
		for exp in -100i32..=100i32 {
			let num = NumberValue::from(exp);
			let fixture = Number::from(num);

			fixture.0.eq_i32(&exp)
		}
	}

	#[test]
	fn number_test() {
		let fixture = create_fixture(42);
		fixture.number().eq_i32(&42)
	}

	#[test]
	fn calc_test() {
		for exp in -100..=100 {
			let fixture = create_fixture(exp);
			fixture.calc().unwrap().eq_i32(&exp)
		}
	}

	#[test]
	fn clone_test() {
		let mut fixture = create_fixture(42);

		let clone = fixture.clone();
		clone.0.eq_i32(&42);

		fixture = create_fixture(100);
		fixture.0.eq_i32(&100);

		clone.0.eq_i32(&42)
	}
}
