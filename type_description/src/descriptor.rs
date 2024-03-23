use crate::extractor::Extractor;
use crate::presenter::Presenter;

pub trait Descriptor {
	const N: usize;
	type Item;
	type Description: ToString;
	type Error;
	
	fn describe(&self, scr: Self::Item) -> Result<Box<[Self::Description; Self::N]>, Self::Error>;
}

pub struct CommonDescriptor<const N: usize, Source, Extraction, Output, Error, T, U>
	where Output: ToString,
	      T: Extractor<N={N}, Source = Source,Output=Extraction,Error=Error>,
	      U: Presenter<N={N},Source=Extraction,Output=Output>
{
	extractor:T,
	presenter:U
}

impl<const N:usize,Source,Extraction,Output,Error,T,U> CommonDescriptor<N,Source,Extraction,Output,Error,T,U>
	where Output: ToString,
	      T: Extractor<N={N}, Source = Source,Output=Extraction,Error=Error>,
	      U: Presenter<N={N},Source=Extraction,Output=Output>
{
	pub fn new(extractor:T,presenter:U)->Self<>{
		CommonDescriptor{extractor,presenter}
	}
}

impl<const N:usize,Source,Extraction,Output,Error,T,U> Descriptor for CommonDescriptor<N,Source,Extraction,Output,Error,T,U>
	where Output: ToString,
	      T: Extractor<N={N}, Source = Source,Output=Extraction,Error=Error>,
	      U: Presenter<N={N},Source=Extraction,Output=Output>
{
	const N: usize = {N};
	type Item = Source;
	type Description = Output;
	type Error = Error;
	
	fn describe(&self, scr: Self::Item) -> Result<Box<[Self::Description; Self::N]>, Self::Error> {
		todo!()
	}
}

#[cfg(test)]
mod tests{
	use std::marker::PhantomData;
	use super::*;
	use std::mem::{MaybeUninit,transmute};
	
	pub struct KeyValuePair(i32,String);
	
	pub enum KeyValue<'a>{
		Key(&'a i32),
		Value(&'a str)
	}
	
	pub struct KvPresenter<'a>(PhantomData<&'a ()>);
	pub struct KvExtractor<'a>(PhantomData<&'a ()>);
	
	impl<'a> Extractor for KvExtractor<'a> {
		const N: usize = 2;
		type Source = &'a KeyValuePair;
		type Output = KeyValue<'a>;
		type Error = ();
		
		fn extract(&self,scr: Self::Source) -> Result<Box<[Self::Output; Self::N]>, Self::Error> {
			let mut boxed=Box::new(MaybeUninit::<Self::Output>::uninit_array::<{Self::N}>());
			
			boxed[0].write(KeyValue::Key(&scr.0));
			boxed[1].write(KeyValue::Value(&scr.1));
			
			Ok(unsafe {transmute::<_,Box<[Self::Output;Self::N]>>(boxed)})
		}
	}
	
	const HEADER:[&str;2]=["X","Y"];
	
	impl<'a> Presenter for KvPresenter<'a> {
		const N: usize = 2;
		type Source = KeyValue<'a>;
		type Output = String;
		
		fn header(&self) -> [&str; Self::N] {
			HEADER
		}
		
		fn present_datum(&self,datum: Self::Source) -> Self::Output {
			match datum {
				KeyValue::Key(k) => k.to_string(),
				KeyValue::Value(v) => v.to_string()
			}
		}
	}
	
	
	
	#[test]
	fn describe_test(){
		let descriptor=CommonDescriptor::new(KvExtractor(PhantomData::default()),KvPresenter(PhantomData::default()));
		
		let sample=KeyValuePair(10,"hello".to_string());
		
		let actual=descriptor.describe(&sample).unwrap();
		
		assert_eq!(actual[0],"10");
		assert_eq!(actual[1],"hello");
	}
	
}