use crate::number_value::NumberValue;

pub trait ArithmeticExpression: Clone {
	fn calc(&self) -> NumberValue;
}
