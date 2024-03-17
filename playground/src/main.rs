#![feature(generic_const_exprs)]
#![feature(associated_const_equality)]

mod const_sample;

trait Some<T, U> {
	fn conv(value: T) -> (T, U);

	fn conv_arr<const N: usize>(arr: [T; N]) -> [(T, U); N] {
		let mut res: [(T, U); N] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

		for (src, dest) in arr.into_iter().zip(res.iter_mut()) {
			*dest = Self::conv(src);
		}

		res
	}
}

struct Foo;

impl Some<i32, f64> for Foo {
	fn conv(value: i32) -> (i32, f64) {
		(value, value as f64)
	}
}

fn main() {
	let a = Foo::conv_arr([1i32, 2, 3]);
}
