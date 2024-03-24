use std::marker::PhantomData;
use crate::extractor::Extractor;
use crate::presenter::Presenter;

pub trait Descriptor<const N: usize, T, U: ToString> {
	fn describe(&self, scr: T) -> Box<[U; N]>;
}

pub struct CommonDescriptor<const N: usize, S, E, O, T, U> {
	extractor: T,
	presenter: U,
	_s: PhantomData<S>,
	_e: PhantomData<E>,
	_o: PhantomData<O>,
}

impl<const N: usize, S, E, O, T, U> CommonDescriptor<N, S, E, O, T, U> where T: Extractor<N, S, E>,
                                                                             U: Presenter<N, E, O>,
                                                                             O: ToString {
	pub fn new(extractor: T, presenter: U) -> Self <> {
		CommonDescriptor { extractor, presenter, _e: PhantomData::default(), _o: PhantomData::default(), _s: PhantomData::default() }
	}
}

impl<const N: usize, S, E, O, T, U> Descriptor<N, S, O> for CommonDescriptor<N, S, E, O, T, U>
	where T: Extractor<N, S, E>,
	      U: Presenter<N, E, O>,
	      O: ToString {
	fn describe(&self, scr: S) -> Box<[O; N]> {
		let arr = self.extractor.extract(scr);
		self.presenter.present(arr)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::mem::{MaybeUninit, transmute};
	
	pub struct KeyValuePair(i32, String);
	
	pub enum KeyValue<'a> {
		Key(&'a i32),
		Value(&'a str),
	}
	
	pub struct KvExtractor<'a>(PhantomData<&'a ()>);
	
	pub struct KvPresenter<'a>(PhantomData<&'a ()>);
	
	const HEADER: [&str; 2] = ["x", "y"];
	
	impl<'a> Presenter<2, KeyValue<'a>, String> for KvPresenter<'a> {
		fn header(&self) -> [&str; 2] {
			HEADER
		}
		
		fn present_datum(&self, datum: KeyValue<'a>) -> String {
			match datum {
				KeyValue::Key(k) => k.to_string(),
				KeyValue::Value(v) => v.to_string()
			}
		}
	}
	
	impl<'a> Extractor<2, &'a KeyValuePair, KeyValue<'a>> for KvExtractor<'a> {
		fn extract(&self, scr: &'a KeyValuePair) -> Box<[KeyValue<'a>; 2]> {
			let mut boxed = Box::new(MaybeUninit::<KeyValue<'a>>::uninit_array::<2>());
			
			boxed[0].write(KeyValue::Key(&scr.0));
			boxed[1].write(KeyValue::Value(&scr.1));
			
			unsafe { transmute::<_, Box<[KeyValue<'a>; 2]>>(boxed) }
		}
	}
	
	#[test]
	fn describe_test() {
		let sample = KeyValuePair(42, "hello world".to_string());
		
		let extractor = KvExtractor(PhantomData::default());
		let presenter = KvPresenter(PhantomData::default());
		
		let descriptor = CommonDescriptor::new(extractor, presenter);
		
		let fixture = descriptor.describe(&sample);
		
		assert_eq!(&fixture[0], "42");
		assert_eq!(&fixture[1], "hello world");
	}
}