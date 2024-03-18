pub type Datum<T, E> = Result<Option<T>, E>;
pub type Data<const N: usize, T, DataErr, DatumErr> = Result<Box<[Datum<T, DatumErr>; N]>, DataErr>;

pub enum DescribeError<DatumErr, PresentationErr> {
	DatumError(DatumErr),
	PresentationError(PresentationErr),
}
pub type PresentationDatum<T, DatumErr, PresentationErr> =
	Datum<(T, String), DescribeError<DatumErr, PresentationErr>>;

pub type PresentationData<const N: usize, T, DataError, DatumError, PresentationError> = Result<
	Box<[Result<Option<(T, String)>, DescribeError<DatumError, PresentationError>>; N]>,
	DataError,
>;

pub trait TypeDescriptor<'a> {
	const SIZE: usize;
	type Target;
	type Output;
	type DataError;
	type DatumError;
	type PresentationError;

	fn explain(
		source: &'a Self::Target,
	) -> Data<{ Self::SIZE }, Self::Output, Self::DataError, Self::DatumError>;

	fn present(
		datum: Datum<Self::Output, Self::DatumError>,
	) -> Result<
		Option<(Self::Output, String)>,
		DescribeError<Self::DatumError, Self::PresentationError>,
	>;

	fn describe(
		source: &'a Self::Target,
	) -> PresentationData<
		{ Self::SIZE },
		Self::Output,
		Self::DataError,
		Self::DatumError,
		Self::PresentationError,
	> {
		let arr = Self::explain(source)?;

		let mut vec: Vec<
			Result<
				Option<(Self::Output, String)>,
				DescribeError<Self::DatumError, Self::PresentationError>,
			>,
		> = Vec::new();

		for elem in arr.into_iter() {
			let a = Self::present(elem);
			vec.push(a);
		}

		let array: Result<
			[Result<
				Option<(Self::Output, String)>,
				DescribeError<Self::DatumError, Self::PresentationError>,
			>; Self::SIZE],
			_,
		> = vec.try_into();

		let array = match array {
			Ok(a) => a,
			Err(_) => unreachable!(),
		};

		return Ok(Box::new(array));
	}
}

#[cfg(test)]
mod tests {
	use crate::type_descriptor::{Data, Datum, DescribeError, TypeDescriptor};

	struct Point {
		x: f64,
		y: f64,
	}

	struct Float<'a>(&'a f64);

	impl<'a> From<&'a f64> for Float<'a> {
		fn from(value: &'a f64) -> Self {
			Float(value)
		}
	}

	struct Conv;

	impl TypeDescriptor<'_> for Conv {
		const SIZE: usize = 2;
		type Target = Point;
		type Output = f64;
		type DataError = String;
		type DatumError = String;
		type PresentationError = String;

		fn explain(
			source: &'_ Self::Target,
		) -> Data<{ Self::SIZE }, Self::Output, Self::DataError, Self::DatumError> {
			Ok(Box::new([Ok(Some(source.x)), Ok(Some(source.y))]))
		}

		fn present(
			datum: Datum<Self::Output, Self::DatumError>,
		) -> Result<
			Option<(Self::Output, String)>,
			DescribeError<Self::DatumError, Self::PresentationError>,
		> {
			let a = datum.unwrap().unwrap();
			let s = a.to_string();
			Ok(Some((a, s)))
		}
	}

	#[test]
	fn compile_test() {
		let p = Point { x: 10.0, y: 42.195 };

		let d = Conv::describe(&p).unwrap();

		for elem in d.iter() {
			if let Ok(a) = elem {
				if let Some(s) = a {
					println!("{}", s.1);
				}
			}
		}
	}
}
