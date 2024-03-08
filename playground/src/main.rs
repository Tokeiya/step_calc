fn main() {}

#[cfg(test)]
mod tests {
	#[test]
	fn test_conditional_return() {
		let mut a = once_cell::sync::OnceCell::<i32>::new();
		let b = a.set(20);
	}
}
