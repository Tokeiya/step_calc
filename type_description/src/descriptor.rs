pub trait Descriptor {
	const N: usize;
	type Source;
	type Output;
	type Error;

	fn describe(scr: Self::Source) -> Result<Box<[Self::Output; { Self::N }]>, Self::Error>;
}
