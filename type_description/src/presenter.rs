use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::mem::{MaybeUninit, transmute};

pub enum AllocationError {
	LayoutError,
	AllocationError,
}

impl Debug for AllocationError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			AllocationError::LayoutError => write!(f, "AllocationError::LayoutError"),
			AllocationError::AllocationError => write!(f, "AllocationError::AllocationError"),
		}
	}
}

impl Display for AllocationError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			AllocationError::LayoutError => write!(f, "LayoutError"),
			AllocationError::AllocationError => write!(f, "AllocationError"),
		}
	}
}

impl Error for AllocationError {}

pub trait Presenter {
	const N: usize;
	type Source;
	type Output: ToString;

	fn present_datum(datum: Self::Source) -> Self::Output;

	fn present(
		data: Box<[Self::Source; { Self::N }]>,
	) -> Result<Box<[Self::Output; { Self::N }]>, AllocationError> {
		let mut boxed = Box::new(MaybeUninit::<Self::Output>::uninit_array::<{ Self::N }>());

		for (idx, elem) in data.into_iter().enumerate() {
			let tmp = Self::present_datum(elem);
			boxed[idx].write(tmp);
		}

		Ok(unsafe { transmute::<_, Box<[Self::Output; { Self::N }]>>(boxed) })
	}
}

#[cfg(test)]
mod tests {
	use std::error::Error;
	use std::fmt::{Debug, Display, Formatter};
	use std::marker::PhantomData;
	use std::mem::{MaybeUninit, transmute};
	
	use crate::descriptor::Descriptor;
	use crate::presenter::Presenter;
	
	pub struct KeyValuePair {
		key: i32,
		value: String,
	}

	pub enum KeyValue<'a> {
		Key(&'a i32),
		Value(&'a str),
	}

	impl KeyValue<'_> {
		pub fn assert_key(&self, expected: i32) {
			assert!(matches!(self,KeyValue::Key(k) if k==&&expected))
		}

		pub fn assert_value(&self, expected: &str) {
			assert!(matches!(self,KeyValue::Value(v) if v==&expected))
		}
	}

	pub struct keyValuePairDescriptor<'a>(PhantomData<&'a ()>);

	pub struct AllocError;

	impl Debug for AllocError {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			write!(f, "Failed to allocation")
		}
	}

	impl Display for AllocError {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			write!(f, "Failed to allocation.")
		}
	}

	impl Error for AllocError {}

	pub struct KeyValuePairDescriptor<'a>(PhantomData<&'a ()>);

	impl<'a> Descriptor for KeyValuePairDescriptor<'a> {
		const N: usize = 2;
		type Source = &'a KeyValuePair;
		type Output = KeyValue<'a>;
		type Error = AllocError;

		fn describe(scr: Self::Source) -> Result<Box<[Self::Output; Self::N]>, Self::Error> {
			let mut boxed = Box::new(MaybeUninit::<Self::Output>::uninit_array::<{ Self::N }>());

			boxed[0].write(KeyValue::Key(&scr.key));
			boxed[1].write(KeyValue::Value(&scr.value));

			let b = unsafe { transmute::<_, Box<[Self::Output; Self::N]>>(boxed) };
			Ok(b)
		}
	}

	pub struct KeyValuePairPresenter<'a>(PhantomData<&'a ()>);

	impl<'a> Presenter for KeyValuePairPresenter<'a> {
		const N: usize = 2;
		type Source = KeyValue<'a>;
		type Output = String;

		fn present_datum(datum: Self::Source) -> Self::Output {
			match datum {
				KeyValue::Key(k) => k.to_string(),
				KeyValue::Value(v) => v.to_string(),
			}
		}
	}

	fn generate(key: i32) -> KeyValuePair {
		KeyValuePair {
			key,
			value: format!("value:{key}"),
		}
	}

	#[test]
	fn describe_test() {
		let data = generate(42);

		let fixture = KeyValuePairDescriptor::describe(&data).unwrap();

		fixture[0].assert_key(42);
		fixture[1].assert_value("value:42");
	}

	#[test]
	fn present_datum_test() {
		let data = generate(42);
		let fixture = KeyValuePairDescriptor::describe(&data).unwrap();

		let [act_key, act_value] = *fixture;

		let actual = KeyValuePairPresenter::present_datum(act_key);
		assert_eq!(&actual, "42");

		let actual = KeyValuePairPresenter::present_datum(act_value);
		assert_eq!(&actual, "value:42");
	}

	#[test]
	fn present_test() {
		let data = generate(42);
		let fixture = KeyValuePairDescriptor::describe(&data).unwrap();

		let fixture = KeyValuePairPresenter::present(fixture).unwrap();
		assert_eq!(2, fixture.len());

		assert_eq!(fixture[0], "42");
		assert_eq!(fixture[1], "value:42");
	}
}