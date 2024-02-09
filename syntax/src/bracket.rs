use crate::arithmetic_expression::ArithmeticExpression;
use crate::expression::Expression;
use crate::number_value::NumberValue;

pub struct Bracket(Box<Expression>);

impl From<Expression> for Bracket {
	fn from(value: Expression) -> Self {
		todo!()
	}
}

impl Clone for Bracket {
	fn clone(&self) -> Self {
		todo!()
	}
}

impl ArithmeticExpression for Bracket {
	fn calc(&self) -> NumberValue {
		todo!()
	}
}
