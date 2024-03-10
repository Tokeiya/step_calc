pub use color::{ColorContext, ConsoleColor, RESET as RESET_NUM};
pub use raw::{
	build_escape, build_string_args, build_string_str, print_args, print_str, println_args,
	println_str, RESET,
};

mod color;
mod raw;
