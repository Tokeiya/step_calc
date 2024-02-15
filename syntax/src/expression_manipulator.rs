use crate::arithmetic_expression::ArithmeticExpression;
use crate::binary_operation::BinaryOperation;
use crate::bracket::Bracket;
use crate::expression::Expression;
use crate::number::Number as NumberExpr;
use crate::number_value::NumberValue;

fn erase_bracket(node: impl ArithmeticExpression) -> Expression {
	let expr = node.to_expression();

	match expr {
		Expression::Number(num) => num.to_expression(),
		Expression::Bracket(bracket) => erase_bracket(bracket.expression().clone()).to_expression(),
		Expression::BinaryOperation(bin) => {
			let left = bin.left().clone();
			let left = erase_bracket(left);

			let right = bin.right().clone();
			let right = erase_bracket(right);

			BinaryOperation::new(left, right, bin.operation().clone()).to_expression()
		}
	}
}

pub fn simplify(root: &impl ArithmeticExpression) -> Expression {
	let tmp = root.clone().to_expression();

	erase_bracket(tmp)
}

#[cfg(test)]
mod tests {
	use crate::arithmetic_expression::ArithmeticExpression;
	use crate::binary_operation::{BinaryOperation, Operation};
	use crate::bracket::Bracket;
	use crate::number::Number as NumberExpr;
	use crate::number_value::NumberValue;

	use super::*;

	#[test]
	fn simple_simplify() {
		let tmp = NumberExpr::from(NumberValue::from(42));
		let tmp = Bracket::from(tmp.to_expression());
		
		let fixture = simplify(&tmp);
		
		tmp.expression().extract_as_number().number().eq_i32(&42);
		fixture.extract_as_number().number().eq_i32(&42);
	}
}
