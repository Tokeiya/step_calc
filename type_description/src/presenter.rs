pub type Datum<T, E> = Result<Option<T>, E>;
pub type Data<'a, const N: usize, T, DataErr, DatumErr> =
	Result<Box<[Datum<T, DatumErr>; N]>, DataErr>;

pub trait Presenter<'a> {
	const SIZE: usize;
	type Target;
	type Output;
	type TypePresentationError;
	type ValuePresentationError;

	fn present(
		scr: &'a Self::Target,
	) -> Data<
		'a,
		{ Self::SIZE },
		Self::Output,
		Self::TypePresentationError,
		Self::ValuePresentationError,
	>;
}
