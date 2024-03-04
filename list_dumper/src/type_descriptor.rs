pub use super::column_info::{Align, ColumnInfo};

pub trait TypeDescriptor<const N:usize> {
	fn columns() -> [ColumnInfo;N] {
		todo!()
	}
	
	fn fill(&self,buffer:&mut [String;N])->bool{
		todo!()
	}

}
