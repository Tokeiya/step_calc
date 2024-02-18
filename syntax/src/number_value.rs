use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Rem, Sub};
pub type NumberResult = Result<NumberValue, ArithmeticError>;
pub struct ArithmeticError {
	rhs: NumberValue,
	lhs: NumberValue,
	message: String,
}

impl ArithmeticError {
	pub fn new(lhs: &NumberValue, rhs: &NumberValue, message: &str) -> Self {
		ArithmeticError {
			rhs: rhs.clone(),
			lhs: lhs.clone(),
			message: message.to_string(),
		}
	}

	pub fn rhs(&self) -> &NumberValue {
		&self.rhs
	}

	pub fn lhs(&self) -> &NumberValue {
		&self.lhs
	}

	pub fn message(&self) -> &str {
		&self.message
	}
}

impl Debug for ArithmeticError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "lhs:{} rhs:{} msg:{}", self.lhs, self.rhs, self.message)?;
		Ok(())
	}
}

impl Display for ArithmeticError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Debug::fmt(self, f)
	}
}

impl Error for ArithmeticError {}

#[derive(PartialEq, Clone)]
pub enum NumberValue {
	Integer(i32),
}

impl Debug for NumberValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			NumberValue::Integer(i) => {
				write!(f, "{}i32", i)
			}
		}
	}
}

impl Display for NumberValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Debug::fmt(self, f)
	}
}

impl From<i32> for NumberValue {
	fn from(value: i32) -> Self {
		Self::Integer(value)
	}
}

impl Add<Self> for NumberValue {
	type Output = NumberResult;

	fn add(self, rhs: NumberValue) -> Self::Output {
		Ok(match self {
			NumberValue::Integer(l) => match rhs {
				NumberValue::Integer(r) => NumberValue::Integer(l + r),
			},
		})
	}
}

impl Sub<Self> for NumberValue {
	type Output = NumberResult;

	fn sub(self, rhs: NumberValue) -> Self::Output {
		Ok(match self {
			NumberValue::Integer(l) => match rhs {
				NumberValue::Integer(r) => NumberValue::Integer(l - r),
			},
		})
	}
}

impl Mul<Self> for NumberValue {
	type Output = NumberResult;

	fn mul(self, rhs: NumberValue) -> Self::Output {
		Ok(match self {
			NumberValue::Integer(l) => match rhs {
				NumberValue::Integer(r) => NumberValue::Integer(l * r),
			},
		})
	}
}

impl Div<Self> for NumberValue {
	type Output = NumberResult;

	fn div(self, rhs: NumberValue) -> Self::Output {
		match rhs {
			NumberValue::Integer(0) => Err(ArithmeticError::new(&self, &rhs, "DIV/0!")),
			_ => Ok(match self {
				NumberValue::Integer(l) => match rhs {
					NumberValue::Integer(r) => NumberValue::Integer(l / r),
				},
			}),
		}
	}
}

impl Rem<Self> for NumberValue {
	type Output = NumberResult;

	fn rem(self, rhs: NumberValue) -> Self::Output {
		Ok(match self {
			NumberValue::Integer(l) => match rhs {
				NumberValue::Integer(r) => NumberValue::Integer(l % r),
			},
		})
	}
}

#[cfg(any(feature = "test_active", test))]
pub mod test_helper {
	use crate::number_value::NumberValue;

	pub fn foo() {
		println!("hello")
	}
	impl NumberValue {
		pub fn eq_i32(&self, expected: &i32) {
			match self {
				NumberValue::Integer(act) => {
					assert_eq!(act, expected)
				}
			}
		}
		pub fn not_eq_i32(&self, expected: &i32) {
			match self {
				NumberValue::Integer(act) => {
					assert_ne!(act, expected)
				}
			}
		}

		pub fn eq_number(&self, expected: &NumberValue) {
			match expected {
				NumberValue::Integer(exp) => self.eq_i32(exp),
			}
		}

		pub fn not_eq_number(&self, expected: &NumberValue) {
			match expected {
				NumberValue::Integer(exp) => self.not_eq_i32(exp),
			}
		}
	}

	#[test]
	#[should_panic]
	fn eq_i32_panic() {
		let act = NumberValue::Integer(100);
		act.eq_i32(&101)
	}

	#[test]
	#[should_panic]
	fn eq_number_panic() {
		let act = NumberValue::Integer(100);
		act.eq_number(&NumberValue::Integer(101))
	}

	#[test]
	fn eq_true_test() {
		for expected in -100..=100 {
			let act = NumberValue::Integer(expected);
			let e = NumberValue::Integer(expected);

			act.eq_i32(&expected);
			act.eq_number(&e);
		}
	}

	#[test]
	#[should_panic]
	fn not_eq_i32_panic() {
		let act = NumberValue::Integer(100);
		act.not_eq_i32(&100);
	}

	#[test]
	#[should_panic]
	fn not_eq_number_panic() {
		let act = NumberValue::Integer(100);
		act.not_eq_number(&NumberValue::Integer(100));
	}

	#[test]
	fn eq_false_test() {
		for exp in -100..=100 {
			let num = NumberValue::Integer(exp);
			num.not_eq_i32(&(exp + 1));

			let num = NumberValue::Integer(exp);
			let num2 = NumberValue::Integer(exp + 1);
			num.not_eq_number(&num2);
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::number_value::NumberValue;

	#[test]
	fn form_test() {
		for exp in -100..=100 {
			NumberValue::from(exp).eq_i32(&exp)
		}
	}

	#[test]
	fn add_test() {
		let a = NumberValue::Integer(10);
		let b = NumberValue::Integer(33);

		(a + b).unwrap().eq_i32(&43)
	}

	#[test]
	fn sub_test() {
		let a = NumberValue::Integer(10);
		let b = NumberValue::Integer(33);

		(a - b).unwrap().eq_i32(&-23)
	}

	#[test]
	fn mul_test() {
		let a = NumberValue::Integer(10);
		let b = NumberValue::Integer(33);

		(a * b).unwrap().eq_i32(&330)
	}

	#[test]
	fn div_test() {
		let a = NumberValue::Integer(88);
		let b = NumberValue::Integer(33);

		(a / b).unwrap().eq_i32(&2)
	}

	#[test]
	fn div_zero_test() {
		let a = NumberValue::Integer(100);
		let b = NumberValue::Integer(0);

		let act = (a / b).err().unwrap();

		act.lhs().eq_i32(&100);
		act.rhs().eq_i32(&0);

		assert_eq!(act.message(), "DIV/0!");
		act.rhs().eq_i32(&0);
		act.lhs().eq_i32(&100);

		let txt = format!("{:?}", act);
		assert_eq!("lhs:100i32 rhs:0i32 msg:DIV/0!", txt);

		let txt = format!("{}", act);
		assert_eq!("lhs:100i32 rhs:0i32 msg:DIV/0!", txt);
	}

	#[test]
	fn rem_test() {
		let a = NumberValue::Integer(88);
		let b = NumberValue::Integer(33);

		(a % b).unwrap().eq_i32(&22)
	}

	#[test]
	fn clone_test() {
		let mut a = NumberValue::Integer(88);
		let b = a.clone();

		a.eq_number(&b);

		a = NumberValue::Integer(33);
		a.not_eq_number(&b);
	}

	#[test]
	fn debug_test() {
		let num = NumberValue::Integer(100);
		let act = format!("{:?}", num);
		assert_eq!("100i32", act)
	}

	#[test]
	fn display_test() {
		let num = NumberValue::Integer(100);
		let act = format!("{}", num);
		assert_eq!("100i32", act)
	}
}
