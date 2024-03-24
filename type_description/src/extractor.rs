pub trait Extractor<const N: usize, S, O> {
	fn extract(&self, scr: S) -> Box<[O; N]>;
}
