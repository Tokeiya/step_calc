pub use super::column_info::{Align, ColumnInfo};

pub trait TypeDescriptor {
	fn columns() -> Vec<ColumnInfo> {
		todo!()
	}


}
