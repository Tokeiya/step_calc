use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub use super::column_info::{Align, ColumnInfo};

pub enum ExprssionError {}

impl Debug for ExprssionError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl Display for ExprssionError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl Error for ExprssionError {}

pub trait TypeDescriptor<const N: usize> {
	fn columns() -> &'static [ColumnInfo; N] {
		todo!()
	}

	fn fill(&self, buffer: &mut [String; N]) -> Result<(), ExprssionError> {
		todo!()
	}

	fn record(&self) -> Result<Vec<String>, ExprssionError> {
		todo!()
	}
}
