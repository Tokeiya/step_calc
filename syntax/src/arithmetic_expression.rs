use crate::expression::Expression;
use crate::number_value::NumberResult;
pub trait ArithmeticExpression: Clone {
	fn calc(&self) -> NumberResult;
	fn to_expression(self) -> Expression;
}
