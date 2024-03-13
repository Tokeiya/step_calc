use std::error::Error as StdError;

use console_qualifier::{ColorContext, ConsoleColor};

use super::alignment::Alignment;
use super::type_descriptor::{DescribeElement, DescribeResult, InnerDescriptor, OuterDescriptor};

pub struct ColumnColorContext(ConsoleColor, ColorContext);

struct ColumnProperty {
	max: Option<usize>,
	alignment: Alignment,
}

pub struct ConsolePrompter<const N: usize> {
	column_properties: Box<[ColumnProperty; N]>,
	packing_size: usize,
}

impl<const N: usize> ConsolePrompter<N> {
	pub fn new(props: Box<[ColumnProperty; N]>, packing_size: usize)->Self {
		ConsolePrompter{
			column_properties:props,
			packing_size
		}
	}

	pub fn properties(&self) -> &[ColumnProperty; N] {
		todo!()
	}
	
	pub fn write_header<T,U,V>()
	where
	  T:InnerDescriptor<SIZE = { N }, GetError = U, ElementError = V>,
	  U:StdError,
	  V:StdError,
	  [();T::SIZE]:
	{
		for elem in T::header().iter() {
			print!("{}\t|",elem);
		}
		println!();
	}
	
	fn write_column<T,U,V>(data:Box<[DescribeElement<V>;{T::SIZE}]>)
	where
	  T:InnerDescriptor<SIZE = { N }, GetError = U, ElementError = V>,
	  U:StdError,
	  V:StdError,
	  [();T::SIZE]:
	{
		for datum in data.iter() {
			match datum {
				Ok(d) => match d {
					None => print!("N/A,"),
					Some(s) => print!("{s},")
				}
				Err(err) => print!("{},",T::element_err_describe(err))
			}
		}
	}
	
	
	fn write_columns<T, U, V>(data:& T)
		where
		  T:InnerDescriptor<SIZE = { N }, GetError = U, ElementError = V>,
		  U:StdError,
		  V:StdError,
		  [();T::SIZE]:
	{
		match data.describe() {
			Ok(value) => {
				Self::write_column::<T,U,V>(value);
				println!();
			}
			Err(err) => println!("{}",T::get_error_describe(&err))
		}
		
	}

	pub fn write_inner<T, U, V>(&self,scr: impl Iterator<Item = T>)
	where
	T:InnerDescriptor<SIZE = { N }, GetError = U, ElementError = V>,
	U:StdError,
	V:StdError,
	[();T::SIZE]:
	{
		Self::write_header();
		
		for row in scr{
			Self::write_columns(&row);
		}
	}
}

#[cfg(test)]
mod tests {
	use std::error::Error;
	use std::fmt::{Debug, Display, Formatter};
	use once_cell::sync::Lazy;
	use crate::type_descriptor::{DescribeResult, InnerDescriptor};
	use anyhow::Error as AnyError;
	use super::*;
	
	pub struct DummyError;
	
	impl Debug for DummyError {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			write!(f,"Dummy Debug")
		}
	}
	
	impl Display for DummyError {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			write!(f,"Dummy Display")
		}
	}
	
	impl Error for DummyError{
		
	}
	
	
	
	pub struct Point(i32,i32);
	
	const POINT_SIZE:usize=2;
	
	
	
	impl InnerDescriptor for &Point{
		const SIZE: usize = POINT_SIZE;
		type GetError = DummyError;
		type ElementError = DummyError;
		
		fn header() -> &'static [&'static str; Self::SIZE] {
			static HEADER:Lazy<[&'static str;POINT_SIZE]> = Lazy::new(||{
				["X","Y"]
			});
			
			&HEADER
		}
		
		fn describe(&self) -> DescribeResult<Self::GetError, Self::ElementError, { Self::SIZE }> {
			let arr=[
				Ok(Some(self.0.to_string())),Ok(Some(self.1.to_string()))
			];
			
			Ok(Box::new(arr))
		}
	}
	
	#[test]
	fn compile_test() {
		let mut vec=Vec::<Point>::new();
		for i in 0..100 {
			vec.push(Point(i,i+1000));
		}
		
		let prop=Box::new([ColumnProperty{max:None,alignment:Alignment::Left},ColumnProperty{max:None,alignment:Alignment::Left}]);
		
		let prompter=ConsolePrompter::new(prop,100);
		
		prompter.write_inner(vec.iter());
		
	}
}
