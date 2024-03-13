use std::error::Error as StdError;
use std::fmt::{Debug, Display};

pub type DescribeElement<E> = Result<Option<String>, E>;
pub type DescribeResult<T, U, const N: usize> = Result<Box<[DescribeElement<U>; N]>, T>;

pub trait InnerDescriptor {
	const SIZE: usize;
	type GetError;
	type ElementError;

	fn header() -> &'static [&'static str; Self::SIZE];
	fn describe(&self) -> DescribeResult<Self::GetError, Self::ElementError, { Self::SIZE }>;

	fn get_error_describe(err: &Self::GetError) -> String {
		String::from("DESC")
	}

	fn element_err_describe(err: &Self::ElementError) -> String {
		String::from("ELEM")
	}
}

pub trait OuterDescriptor {
	const SIZE: usize;
	type GetError: StdError;
	type ElementError: StdError;

	fn header() -> Box<[&'static str; Self::SIZE]>;
	fn describe(value: &Self)
		-> DescribeResult<Self::GetError, Self::ElementError, { Self::SIZE }>;

	fn get_error_describe(err: &Self::GetError) -> String {
		format!("{}", err)
	}

	fn element_err_describe(err: &Self::ElementError) -> String {
		format!("{}", err)
	}
}
