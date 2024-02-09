use crate::arithmetic_expression::ArithmeticExpression;
use crate::number_value::NumberValue;

pub struct Number(NumberValue);
impl From<NumberValue> for Number {
	fn from(value: NumberValue) -> Self {
		Number(value)
	}
}

impl ArithmeticExpression for Number {
	fn calc(&self) -> NumberValue {
		self.0.clone()
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
			fixture.calc().eq_i32(&exp)
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
