pub enum Align {
	Left,
	Center,
	Right,
}

pub struct ColumnInfo {
	header: String,
	align: Align,
	max_len: Option<usize>,
	min_len: Option<usize>,
}

impl ColumnInfo {
	pub fn new(header: &str, align: Align, max_len: Option<usize>, min_len: Option<usize>) -> Self {
		todo!()
	}

	pub fn header(&self) -> &str {
		todo!()
	}

	pub fn align(&self) -> &Align {
		todo!()
	}

	pub fn max_len(&self) -> &Option<usize> {
		todo!()
	}

	pub fn min_len(&self) -> &Option<usize> {
		todo!()
	}
}
