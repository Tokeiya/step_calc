use std::error::Error as StdError;
use std::fmt::{Debug, Display};

pub type DescribeElement<T> = Result<Option<String>, T>;
pub type DescribeResult<T, U, const N: usize> = Result<Box<[DescribeElement<U>; N]>, T>;

pub trait InnerDescriptor {
	const SIZE: usize;

	fn header() -> Box<[&'static str; Self::SIZE]>;
	fn describe<T: StdError, U: StdError>(&self) -> DescribeResult<T, U, { Self::SIZE }>;
}

pub trait OuterDescriptor {
	const SIZE: usize;

	fn header() -> Box<[&'static str; Self::SIZE]>;
	fn describe<T: StdError, U: StdError>(value: &Self) -> DescribeResult<T, U, { Self::SIZE }>;
}
