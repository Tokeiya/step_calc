#![feature(generic_const_exprs)]
#![feature(associated_const_equality)]

use std::alloc::{alloc, Layout};
use std::marker::PhantomData;
use std::ptr;

trait Foo{
	const N:usize;
	type Item;
}

trait Bar{
	const N:usize;
	type Item;
}

pub struct Hoge<const N:usize,T,U:Foo<N={N},Item=T>,V:Bar<N={N},Item=T>>(PhantomData<T>,PhantomData<U>,PhantomData<V>);

fn main() {
}
