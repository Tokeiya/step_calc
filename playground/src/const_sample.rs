

trait ArrayReturn {
	const SIZE: usize;
	
	fn get_array(&self) -> Box<[usize; Self::SIZE]>;
}

struct Array42;

impl ArrayReturn for Array42 {
	const SIZE: usize = 42;
	
	fn get_array(&self) -> Box<[usize; Self::SIZE]> {
		let mut arr = [0usize; Self::SIZE];
		
		for elem in 0..Self::SIZE {
			arr[elem] = elem;
		}
		
		Box::new(arr)
	}
}

fn f<const N: usize, T>(value: &T)
	where
	  T:ArrayReturn<SIZE={N}>,
	  [(); T::SIZE]:,
{
	let array = value.get_array();
	
	for elem in array.iter() {
		print!("{}\t", elem);
	}
	
	
	
}

fn proc() {
	let a = Array42 {};
	f(&a)
}