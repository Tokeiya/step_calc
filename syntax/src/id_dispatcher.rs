use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum IdDispatcherError {
	EmptyParent,
	Wraparound,
}

impl Debug for IdDispatcherError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let msg = match self {
			IdDispatcherError::EmptyParent => "Parent is empty.",
			IdDispatcherError::Wraparound => "wraparound",
		};

		write!(f, "{msg}")
	}
}

impl Display for IdDispatcherError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl Error for IdDispatcherError {}

type IdResult = Result<usize, IdDispatcherError>;

pub struct IdDispatcher(VecDeque<usize>, Option<usize>);

impl IdDispatcher {
	pub fn new() -> Self {
		todo!()
	}

	pub fn current(&self) -> IdResult {
		todo!()
	}

	pub fn parent(&self) -> IdResult {
		todo!()
	}

	pub fn get(&mut self) -> IdResult {
		todo!()
	}

	pub fn pop(&mut self) -> IdResult {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		let fixture = IdDispatcher::new();
		assert_eq!(0, fixture.0.len());
		assert_eq!(0, fixture.1.unwrap());
	}

	#[test]
	fn current() {
		let mut fixture = IdDispatcher::new();

		matches!(
			fixture.current().expect_err("unreachable!"),
			IdDispatcherError::EmptyParent
		);

		let get = fixture.get().unwrap();
		assert_eq!(fixture.current().unwrap(), 1);
		assert_eq!(fixture.current().unwrap(), get);

		fixture.1 = Some(usize::MAX);
		_ = fixture.get();

		matches!(
			fixture.current().expect_err(""),
			IdDispatcherError::Wraparound
		);
	}

	#[test]
	fn parent() {
		let mut fixture = IdDispatcher::new();
		matches!(
			fixture.parent().expect_err(""),
			IdDispatcherError::EmptyParent
		);

		let recent = fixture.get().unwrap();
		matches!(
			fixture.parent().expect_err(""),
			IdDispatcherError::EmptyParent
		);

		_ = fixture.get();
	}
}
