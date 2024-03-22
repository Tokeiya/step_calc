#![feature(generic_const_exprs)]
#![feature(associated_const_equality)]

use std::alloc::{alloc, Layout};
use std::ptr;

unsafe fn f<T>(first: T, second: T) -> Box<[T; 2]> {
	// [T; 2]のための正しいメモリレイアウトを計算
	let layout = Layout::array::<T>(2).unwrap();

	// レイアウトに基づいてヒープ上にメモリを確保
	let arr_ptr = alloc(layout) as *mut T;

	// 配列の要素を初期化
	ptr::write(arr_ptr.add(0), first);
	ptr::write(arr_ptr.add(1), second);

	// 生ポインタからBox<[T; 2]>を生成
	Box::from_raw(arr_ptr as *mut [T; 2])
}
fn main() {
	let a = unsafe { f(10.to_string(), 20.to_string()) };
	for elem in a.into_iter() {
		println!("{elem}")
	}
}
