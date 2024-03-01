use std::collections::VecDeque;
use syntax::arithmetic_expression::ArithmeticExpression;
use syntax::binary_operation::{BinaryOperation, Operation};
use syntax::expression::Expression;
use syntax::number::Number as NumExpr;

pub use super::tokenizer::tokenize;
pub use super::tokenizer::Token;

fn build_binary(operator: &Operation, stack: &mut Vec<Expression>) -> bool {
	if stack.len() >= 2 {
		let right = stack.pop().unwrap();
		let left = stack.pop().unwrap();

		let bin = BinaryOperation::new(left, right, operator.clone());
		stack.push(bin.to_expression());
		true
	} else {
		false
	}
}

pub fn step_calc(input: &mut VecDeque<Token>, stack: &mut Vec<Expression>) -> bool {
	if !input.is_empty() {
		let current = &input[input.len() - 1];
		match current {
			Token::Number(num) => {
				println!("push:{}", num);
				stack.push(Expression::Number(NumExpr::from(num.clone())));
				input.pop_back();
				true
			}
			Token::Operator(op) => build_binary(op, stack),
		}
	} else {
		false
	}
}

#[cfg(test)]
mod tests {
	use crate::rpn::parser::{step_calc, tokenize, Token};
	use syntax::arithmetic_expression::ArithmeticExpression;
	use syntax::binary_operation::Operation;
	use syntax::expression::Expression;

	#[test]
	fn step_test() {
		let mut input = tokenize("10 20 -").0;
		let mut stack = Vec::<Expression>::default();

		assert!(step_calc(&mut input, &mut stack));
		Expression::extract_as_number(&stack[0])
			.number()
			.eq_i32(&10);
		assert_eq!(stack.len(), 1);

		assert!(step_calc(&mut input, &mut stack));
		let a = stack[0].extract_as_number();

		println!("{:?}", input);
		println!("{:?}", a.number());
		println!("{}", stack.len());

		stack[1].extract_as_number().number().eq_i32(&20);
		assert_eq!(stack.len(), 2);

		assert!(step_calc(&mut input, &mut stack));
		assert_eq!(stack.len(), 1);

		let tmp = stack[0].extract_as_binary_operation();
		tmp.left().extract_as_number().number().eq_i32(&10);
		tmp.right().extract_as_number().number().eq_i32(&20);

		matches!(tmp.operation(), Operation::Sub);

		let result = tmp.calc().unwrap();
		result.eq_i32(&-10);
	}
}
