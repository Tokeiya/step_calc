#![feature(generic_const_exprs)]
#![feature(associated_const_equality)]

extern crate core;

pub use type_descriptor::{
	Data, Datum, DescribeError, PresentationData, PresentationDatum, TypeDescriptor,
};

mod alignment;
mod type_descriptor;
