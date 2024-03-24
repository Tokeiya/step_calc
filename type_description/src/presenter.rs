use std::mem::{MaybeUninit, transmute};

pub trait Presenter<const N: usize, S, O> {
	fn header(&self) -> [&str; N];
	
	fn present_datum(&self, datum: S) -> O;
	
	fn present(&self, data: Box<[S; N]>) -> Box<[O; N]> {
		let mut boxed = Box::new(MaybeUninit::<O>::uninit_array::<{ N }>());
		
		for (idx, elem) in data.into_iter().enumerate() {
			let tmp = self.present_datum(elem);
			boxed[idx].write(tmp);
		}
		
		unsafe { transmute::<_, Box<[O; N]>>(boxed) }
	}
}

#[cfg(test)]
mod tests {
	use std::error::Error;
	use std::fmt::{Debug, Display, Formatter};
	use std::marker::PhantomData;
	use std::mem::{MaybeUninit, transmute};
	
	use crate::extractor::Extractor;
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
	
	pub struct KeyValuePairExtractor<'a>(PhantomData<&'a ()>);
	
	impl<'a> Extractor<2, &'a KeyValuePair, KeyValue<'a>> for KeyValuePairExtractor<'a> {
		fn extract(&self, scr: &'a KeyValuePair) -> Box<[KeyValue<'a>; 2]> {
			let mut boxed = Box::new(MaybeUninit::<KeyValue<'a>>::uninit_array::<2>());
			
			boxed[0].write(KeyValue::Key(&scr.key));
			boxed[1].write(KeyValue::Value(&scr.value));
			
			let b = unsafe { transmute::<_, Box<[KeyValue<'a>; 2]>>(boxed) };
			b
		}
	}
	
	const HEADER: [&'static str; 2] = ["X", "Y"];
	
	pub struct KeyValuePairPresenter<'a>(PhantomData<&'a ()>);
	
	
	impl<'a> Presenter<2, KeyValue<'a>, String> for KeyValuePairPresenter<'a> {
		fn header(&self) -> [&str; 2] {
			HEADER
		}
		
		fn present_datum(&self, datum: KeyValue<'a>) -> String {
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
	fn header_test() {
		let presenter = KeyValuePairPresenter(PhantomData::default());
		let a = presenter.header();
		
		assert_eq!(a[0], "X");
		assert_eq!(a[1], "Y");
	}
	
	
	#[test]
	fn describe_test() {
		let extractor = KeyValuePairExtractor(PhantomData::default());
		
		let data = generate(42);
		
		let fixture = extractor.extract(&data);
		
		fixture[0].assert_key(42);
		fixture[1].assert_value("value:42");
	}
	
	#[test]
	fn present_datum_test() {
		let extractor = KeyValuePairExtractor(PhantomData::default());
		let presenter = KeyValuePairPresenter(PhantomData::default());
		
		
		let data = generate(42);
		let fixture = extractor.extract(&data);
		
		let [act_key, act_value] = *fixture;
		
		let actual = presenter.present_datum(act_key);
		assert_eq!(&actual, "42");
		
		let actual = presenter.present_datum(act_value);
		assert_eq!(&actual, "value:42");
	}
	
	#[test]
	fn present_test() {
		let extractor = KeyValuePairExtractor(PhantomData::default());
		let presenter = KeyValuePairPresenter(PhantomData::default());
		
		let data = generate(42);
		let fixture = extractor.extract(&data);
		
		let fixture = presenter.present(fixture);
		assert_eq!(2, fixture.len());
		
		assert_eq!(fixture[0], "42");
		assert_eq!(fixture[1], "value:42");
	}
}
