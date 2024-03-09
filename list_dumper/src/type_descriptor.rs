use super::alignment::Alignment;

pub trait InnerDescriptor {
	const SIZE: usize;
	fn describe(&self) -> Vec<(Option<String>, Alignment)>;
}

pub trait OuterDescriptor {
	const SIZE: usize;
	fn describe(value: &Self) -> Vec<(Option<String>, Alignment)>;
}

pub trait Hoge {
	const S: usize;
	fn foo() -> [u8; Self::S];
}
