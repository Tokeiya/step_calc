use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum IdDispatcherError {
	EmptyParent,
	EmptyCurrent,
	Wraparound,
}

impl Debug for IdDispatcherError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let msg = match self {
			IdDispatcherError::EmptyParent => "Parent is empty.",
			IdDispatcherError::Wraparound => "Wraparound.",
			IdDispatcherError::EmptyCurrent => "Current is empty.",
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

pub type IdResult = Result<usize, IdDispatcherError>;

pub struct IdDispatcher(Vec<usize>, Option<usize>);

impl IdDispatcher {
	pub fn new() -> Self {
		IdDispatcher(Vec::new(), Some(0))
	}

	pub fn current(&self) -> IdResult {
		Ok(*self.0.last().ok_or(IdDispatcherError::EmptyCurrent)?)
	}

	pub fn parent(&self) -> IdResult {
		if self.0.len() >= 2 {
			Ok(self.0[self.0.len() - 2])
		} else {
			Err(IdDispatcherError::EmptyParent)
		}
	}

	pub fn get(&mut self) -> IdResult {
		fn proc(v: &Option<usize>) -> IdResult {
			1usize
				.checked_add(v.ok_or(IdDispatcherError::Wraparound)?)
				.ok_or(IdDispatcherError::Wraparound)
		}

		match proc(&self.1) {
			Ok(v) => {
				self.1 = Some(v);
				self.0.push(v);
				Ok(v)
			}
			Err(_) => Err(IdDispatcherError::Wraparound),
		}
	}

	pub fn pop(&mut self) -> IdResult {
		self.0.pop().ok_or(IdDispatcherError::EmptyCurrent)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use once_cell::sync::Lazy;
	use std::ops::Deref;

	static ERRORS: Lazy<[IdDispatcherError; 3]> = Lazy::new(|| {
		[
			IdDispatcherError::EmptyCurrent,
			IdDispatcherError::Wraparound,
			IdDispatcherError::EmptyParent,
		]
	});

	#[test]
	fn debug() {
		for err in ERRORS.deref() {
			let actual = format!("{:?}", err);

			match err {
				IdDispatcherError::EmptyParent => assert_eq!(actual, "Parent is empty."),
				IdDispatcherError::EmptyCurrent => assert_eq!(actual, "Current is empty."),
				IdDispatcherError::Wraparound => assert_eq!(actual, "Wraparound."),
			}
		}
	}

	#[test]
	fn display() {
		for err in ERRORS.deref() {
			let actual = format!("{}", err);

			match err {
				IdDispatcherError::EmptyParent => assert_eq!(actual, "Parent is empty."),
				IdDispatcherError::EmptyCurrent => assert_eq!(actual, "Current is empty."),
				IdDispatcherError::Wraparound => assert_eq!(actual, "Wraparound."),
			}
		}
	}

	#[test]
	fn new() {
		let fixture = IdDispatcher::new();
		assert_eq!(0, fixture.0.len());
		assert_eq!(0, fixture.1.unwrap());
	}

	#[test]
	fn after_initial() {
		let mut fixture = IdDispatcher::new();

		assert!(matches!(
			fixture.current().expect_err("unreachable!"),
			IdDispatcherError::EmptyCurrent
		));

		assert!(matches!(
			fixture.parent().expect_err(""),
			IdDispatcherError::EmptyParent
		));
	}

	#[test]
	fn wrap_around() {
		let mut fixtuire = IdDispatcher::new();
		fixtuire.1 = Some(usize::MAX - 1);

		assert_eq!(fixtuire.get().unwrap(), usize::MAX);

		assert!(matches!(fixtuire.get(), Err(IdDispatcherError::Wraparound)));
		assert_eq!(fixtuire.current().unwrap(), usize::MAX);
	}

	#[test]
	fn normal_iterative() {
		//continue stack test.
		let mut fixute = IdDispatcher::new();
		assert_eq!(fixute.get().unwrap(), 1);

		for expected in 2..100usize {
			assert_eq!(fixute.get().unwrap(), expected);
			assert_eq!(fixute.current().unwrap(), expected);
			assert_eq!(fixute.parent().unwrap(), expected - 1);
		}

		//pop and push test.
		fixute = IdDispatcher::new();
		_ = fixute.get().unwrap();

		for expected in 2..100usize {
			assert_eq!(fixute.get().unwrap(), expected);
			assert_eq!(fixute.current().unwrap(), expected);
			assert_eq!(fixute.parent().unwrap(), 1);

			assert_eq!(fixute.pop().unwrap(), expected);
			assert_eq!(fixute.current().unwrap(), 1);
		}
	}
}
