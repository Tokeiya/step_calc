pub trait Extractor {
	const N: usize;
	type Source;
	type Output;
	type Error;
	
	fn extract(&self, scr: Self::Source) -> Result<Box<[Self::Output; { Self::N }]>, Self::Error>;
}
