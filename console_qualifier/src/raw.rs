use std::fmt::Arguments;

#[cfg(test)]
use dashmap::DashSet;
#[cfg(test)]
use once_cell::sync::Lazy;

use super::color::{ColorContext, ConsoleColor};

pub const PREFIX: &str = r"\x1B[";
pub const POSTFIX: &str = "m";
pub const RESET: &str = r"\x1B[0m";
#[cfg(test)]
static ACTUAL: Lazy<DashSet<String>> = Lazy::new(|| DashSet::default());

#[cfg(test)]
fn insert(actual: String, is_ln: bool) {
	if is_ln {
		std::println!("{}", &actual)
	} else {
		std::print!("{}", &actual)
	}

	if !ACTUAL.insert(actual) {
		panic!("Already inserted!")
	}
}

macro_rules! println {
    () => {
	    unreachable!()
    };
	($($arg:tt)*)=>{
		#[cfg(not(test))]{
			println!($($arg)*)
		}

		#[cfg(test)]{
			insert(format!($($arg)*),true)
		}

	};
}

macro_rules! print {
    () => {
	    unreachable!()
    };
	($($arg:tt)*)=>{
		#[cfg(not(test))]{
			print!($($arg)*)
		}

		#[cfg(test)]{
			insert(format!($($arg)*),false)
		}
	}
}

pub fn build_escape(color: Option<&ConsoleColor>, context: &ColorContext) -> String {
	match color {
		None => String::default(),
		Some(col) => match context {
			ColorContext::Foreground => format!(r"\x1B[{}m", col.foreground()),
			ColorContext::Background => format!(r"\x1B[{}m", col.background()),
		},
	}
}

pub fn build_string_str(
	foreground: Option<&ConsoleColor>,
	background: Option<&ConsoleColor>,
	content: &str,
) -> String {
	let mut str=String::from(content);
	add_color(foreground,background,&mut str);
	return str;
}

pub fn build_string_args(
	foreground: Option<&ConsoleColor>,
	background: Option<&ConsoleColor>,
	arg: std::fmt::Arguments<'_>,
) -> String {
	let  mut str=format!("{}",arg);
	add_color(foreground,background,&mut str);
	str
}

pub fn add_color(
	foreground: Option<&ConsoleColor>,
	background: Option<&ConsoleColor>,
	text: &mut String,
) {
	text.pu
	
}

pub fn println_str(
	foreground: Option<&ConsoleColor>,
	background: Option<&ConsoleColor>,
	content: &str,
) {
	todo!()
}

pub fn println_args(
	foreground: Option<&ConsoleColor>,
	background: Option<&ConsoleColor>,
	args: Arguments<'_>,
) {
	todo!()
}

pub fn print_str(
	foreground: Option<&ConsoleColor>,
	background: Option<&ConsoleColor>,
	content: &str,
) {
	todo!()
}

pub fn print_args(
	foreground: Option<&ConsoleColor>,
	background: Option<&ConsoleColor>,
	args: Arguments<'_>,
) {
	todo!()
}

#[cfg(test)]
mod tests {
	use std::sync::atomic::{AtomicUsize, Ordering};
	use std::sync::atomic::Ordering::Relaxed;
	
	use once_cell::sync::Lazy;
	
	use super::*;
	
	static COLORS: Lazy<[ConsoleColor; 16]> = Lazy::new(|| {
		[
			ConsoleColor::Black,
			ConsoleColor::Red,
			ConsoleColor::Green,
			ConsoleColor::Yellow,
			ConsoleColor::Blue,
			ConsoleColor::Magenta,
			ConsoleColor::Cyan,
			ConsoleColor::White,
			ConsoleColor::Gray,
			ConsoleColor::BrightRed,
			ConsoleColor::BrightGreen,
			ConsoleColor::BrightYellow,
			ConsoleColor::BrightBlue,
			ConsoleColor::BrightMagenta,
			ConsoleColor::BrightCyan,
			ConsoleColor::BrightWhite,
		]
	});

	static ID_SEED: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::default());

	fn build_expected_str(
		foreground: Option<&ConsoleColor>,
		background: Option<&ConsoleColor>,
		id: usize,
		text: &str,
	) -> String {
		format!(
			r"{}{}{}{}\x1B[0m",
			build_escape(foreground, &ColorContext::Foreground),
			build_escape(background, &ColorContext::Background),
			id,
			text
		)
	}

	fn build_expected_args(
		foreground: Option<&ConsoleColor>,
		background: Option<&ConsoleColor>,
		id: usize,
		args: Arguments,
	) -> String {
		format!(
			r"{}{}{}{}\x1B[0m",
			build_escape(foreground, &ColorContext::Foreground),
			build_escape(background, &ColorContext::Background),
			id,
			args
		)
	}

	fn concat_id(id: usize, txt: &str) -> String {
		format!("{}{}", id, txt)
	}

	fn assert_output(expected: &str) {
		assert!(ACTUAL.remove(expected).is_some())
	}

	#[test]
	fn build_escape_test() {
		for elem in COLORS.iter().map(Some) {
			assert_eq!(
				build_escape(elem, &ColorContext::Foreground),
				format!(r"\x1B[{}m", elem.unwrap().foreground())
			);
			assert_eq!(
				build_escape(elem, &ColorContext::Background),
				format!(r"\x1B[{}m", elem.unwrap().background())
			);
		}

		assert_eq!(build_escape(None, &ColorContext::Foreground), "");
		assert_eq!(build_escape(None, &ColorContext::Background), "");
	}

	#[test]
	fn build_string_str_test() {
		for foreground in COLORS.iter().map(Some) {
			for background in COLORS.iter().map(Some) {
				let id = ID_SEED.fetch_add(1, Ordering::Relaxed);
				let act =
					build_string_str(foreground, background, &format!("{}{}", id, "hello world"));
				assert_eq!(
					act,
					build_expected_str(foreground, background, id, "hello world")
				);
			}

			let id = ID_SEED.fetch_add(1, Ordering::Relaxed);
			let act = build_string_str(None, foreground, &format!("{}{}", id, "Foreground NONE"));
			assert_eq!(
				act,
				build_expected_str(None, foreground, id, "Foreground NONE")
			);

			let id = ID_SEED.fetch_add(1, Ordering::Relaxed);
			let act = build_string_str(foreground, None, &format!("{}{}", id, "Background NONE"));
			assert_eq!(
				act,
				build_expected_str(foreground, None, id, "Background NONE")
			);
		}

		let act = build_string_str(None, None, "42NONE");
		assert_eq!(act, build_expected_str(None, None, 42, "NONE"));
	}

	#[test]
	fn build_string_args_test() {
		for foreground in COLORS.iter().map(Some) {
			for background in COLORS.iter().map(Some) {
				let id = ID_SEED.fetch_add(1, Relaxed);
				assert_eq!(
					build_string_args(foreground, background, format_args!("{}{}", id, "ARGS")),
					build_expected_str(foreground, background, id, "ARGS")
				);
			}

			let id = ID_SEED.fetch_add(1, Relaxed);
			assert_eq!(
				build_string_args(foreground, None, format_args!("{}{}", id, "BackgroundNone")),
				build_expected_str(foreground, None, id, "BackgroundNone")
			);

			let id = ID_SEED.fetch_add(1, Relaxed);
			assert_eq!(
				build_string_args(None, foreground, format_args!("{}{}", id, "ForegroundNone")),
				build_expected_str(None, foreground, id, "ForegroundNone")
			);
		}
	}

	#[test]
	fn add_color_test() {
		for fore in COLORS.iter().map(Some) {
			for back in COLORS.iter().map(Some) {
				let mut str = String::from("42add");
				add_color(fore, back, &mut str);
				assert_eq!(&str, &build_expected_str(fore, back, 42, "add"));
			}

			let mut str = String::from("42BackNone");

			add_color(fore, None, &mut str);
			assert_eq!(&str, &build_expected_str(fore, None, 42, "BackNone"));

			str.clear();
			str.push_str("42ForeNone");
			add_color(None, fore, &mut str);
			assert_eq!(&str, &build_expected_str(None, fore, 42, "ForeNone"));
		}

		let mut str = String::from("42BothNone");
		add_color(None, None, &mut str);
		assert_eq!(&str, &build_expected_str(None, None, 42, "BothNone"));
	}

	#[test]
	fn println_test() {
		for fore in COLORS.iter().map(Some) {
			for back in COLORS.iter().map(Some) {
				let id = ID_SEED.fetch_add(1, Relaxed);
				println_str(fore, back, &format!("{}{}", id, "BOTH"));
				assert_output(&build_expected_str(fore, back, id, "BOTH"));
			}

			let id = ID_SEED.fetch_add(1, Relaxed);
			println_str(fore, None, &format!("{}{}", id, "BackNone"));
			assert_output(&build_expected_str(fore, None, id, "BackNone"));

			let id = ID_SEED.fetch_add(1, Relaxed);
			println_str(None, fore, &format!("{}{}", id, "ForeNone"));
			assert_output(&build_expected_str(None, fore, id, "ForeNone"));
		}
	}

	#[test]
	fn println_args_test() {
		for fore in COLORS.iter().map(Some) {
			for back in COLORS.iter().map(Some) {
				let id = ID_SEED.fetch_add(1, Relaxed);
				println_args(fore, back, format_args!("{}{}", id, "ARG_BOTH"));
				assert_output(&build_expected_str(fore, back, id, "ARG_BOTH"));
			}

			let id = ID_SEED.fetch_add(1, Relaxed);
			println_args(fore, None, format_args!("{}{}", id, "ARG_BACK_NONE"));
			assert_output(&build_expected_str(fore, None, id, "ARG_BACK_NONE"));

			let id = ID_SEED.fetch_add(1, Relaxed);
			println_args(None, fore, format_args!("{}{}", id, "ARG_FORE_NONE"));
			assert_output(&build_expected_str(None, fore, id, "ARG_FORE_NONE"));
		}
	}
}
