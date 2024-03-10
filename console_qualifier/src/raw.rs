use std::fmt::Arguments;

#[cfg(test)]
use dashmap::DashSet;
use once_cell::sync::Lazy;
use regex::Regex;

use super::color::ConsoleColor;

pub const RESET: &str = "\x1B[0m";

#[cfg(test)]
static PRINT_LN_ACTUAL: Lazy<DashSet<String>> = Lazy::new(DashSet::default);

#[cfg(test)]
static PRINT_ACTUAL: Lazy<DashSet<String>> = Lazy::new(DashSet::default);

#[cfg(test)]
static IDENTITY: &str = "<<contain>>";

#[cfg(test)]
macro_rules! println {
    () => {
        unreachable!();
    };
    ($($arg:tt)*) => ({
        let txt = format!($($arg)*);
        std::println!("{}", &txt);

        if txt.contains(IDENTITY){
	        if !PRINT_LN_ACTUAL.insert(txt) {
	            panic!("Already inserted");
	        }
        }
    });
}

#[cfg(test)]
macro_rules! print {
    () => {
        unreachable!();
    };
    ($($arg:tt)*) => ({
        let txt = format!($($arg)*);
        std::print!("{}", &txt);

        if !PRINT_ACTUAL.insert(txt) {
            panic!("Already inserted");
        }
    });
}

#[cfg(test)]
fn assert_ln_output(expected: &str) {
	assert!(PRINT_LN_ACTUAL.remove(expected).is_some())
}

#[cfg(test)]
fn assert_output(expected: &str) {
	assert!(PRINT_ACTUAL.remove(expected).is_some(), "MISS:{}", expected);
}

pub fn build_escape(
	foreground: &Option<&ConsoleColor>,
	background: &Option<&ConsoleColor>,
) -> String {
	let mut str = String::default();

	if let Some(fore) = foreground {
		str.push_str(&format!("\x1B[{}m", fore.foreground()));
	}

	if let Some(back) = background {
		str.push_str(&format!("\x1B[{}m", back.background()));
	}

	str
}

pub fn build_string_str(
	foreground: &Option<&ConsoleColor>,
	background: &Option<&ConsoleColor>,
	content: &str,
) -> String {
	let mut str = build_escape(foreground, background);
	str.push_str(content);
	str.push_str(RESET);

	str
}

pub fn build_string_args(
	foreground: &Option<&ConsoleColor>,
	background: &Option<&ConsoleColor>,
	arg: std::fmt::Arguments<'_>,
) -> String {
	let mut str = build_escape(foreground, background);
	str.push_str(&format!("{}", arg));
	str.push_str(RESET);

	str
}

pub fn println_str(
	foreground: &Option<&ConsoleColor>,
	background: &Option<&ConsoleColor>,
	content: &str,
) {
	let mut str = build_escape(foreground, background);
	str.push_str(content);
	str.push_str(RESET);

	println!("{}", str)
}

pub fn println_args(
	foreground: &Option<&ConsoleColor>,
	background: &Option<&ConsoleColor>,
	args: Arguments<'_>,
) {
	println!("{}{}{}", build_escape(foreground, background), args, RESET)
}

pub fn print_str(
	foreground: &Option<&ConsoleColor>,
	background: &Option<&ConsoleColor>,
	content: &str,
) {
	print!(
		"{}{}{}",
		build_escape(foreground, background),
		content,
		RESET
	)
}

pub fn print_args(
	foreground: &Option<&ConsoleColor>,
	background: &Option<&ConsoleColor>,
	args: Arguments<'_>,
) {
	print!("{}{}{}", build_escape(foreground, background), args, RESET)
}

pub fn remove_color_definition(scr: &str) -> String {
	static reg: Lazy<Regex> = Lazy::new(|| Regex::new(r"\x1B\[.+?m").unwrap());
	let array: Vec<_> = reg.find_iter(scr).collect();

	if array.is_empty() {
		scr.to_string()
	} else {
		let mut ret = String::default();
		let mut s = 0usize;

		for elem in array.iter() {
			ret.push_str(&scr[s..elem.start()]);
			s = elem.end();
		}
		ret
	}
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

	fn all_combination() -> impl Iterator<Item = (&'static ConsoleColor, &'static ConsoleColor)> {
		COLORS
			.iter()
			.flat_map(|x| COLORS.iter().map(move |y| (x, y)))
	}

	static ID_SEED: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::default());

	fn build_expected_str(
		foreground: &Option<&ConsoleColor>,
		background: &Option<&ConsoleColor>,
		id: usize,
		text: &str,
	) -> String {
		format!(
			"{}<<contain>>{}{}\x1B[0m",
			build_escape(&foreground, &background),
			id,
			text
		)
	}

	fn concat_id(id: usize, txt: &str) -> String {
		format!("{}{}", id, txt)
	}

	#[test]
	fn remove_color_definition_test() {
		let input: &str = "\x1B[96m\x1B[101mhello\n \x1B[92m\x1B[103mworld\x1B[0m";
		assert_eq!(remove_color_definition(input), "hello\n world");

		assert_eq!(remove_color_definition(""), "");
		assert_eq!(remove_color_definition("\x1B[96m\x1B[101m"), "");
		assert_eq!(remove_color_definition("hello\t world"), "hello\t world");
	}

	#[test]
	fn consistency_test() {
		println!("{}", "<<contain>>foo");
		assert_ln_output("<<contain>>foo");

		print!("{}", "<<contain>>bar");
		assert_output("<<contain>>bar");
	}

	#[test]
	fn build_escape_test() {
		for (fore, back) in all_combination() {
			let expected = format!("\x1B[{}m\x1B[{}m", fore.foreground(), back.background());
			assert_eq!(build_escape(&Some(fore), &Some(back)), expected);
		}
	}

	#[test]
	fn build_string_str_test() {
		for foreground in COLORS.iter().map(Some) {
			for background in COLORS.iter().map(Some) {
				let id = ID_SEED.fetch_add(1, Ordering::Relaxed);
				let act = build_string_str(
					&foreground,
					&background,
					&format!("<<contain>>{}{}", id, "hello world"),
				);

				assert_eq!(
					act,
					build_expected_str(&foreground, &background, id, "hello world")
				);
			}

			let id = ID_SEED.fetch_add(1, Ordering::Relaxed);
			let act = build_string_str(
				&None,
				&foreground,
				&format!("<<contain>>{}{}", id, "Foreground NONE"),
			);
			assert_eq!(
				act,
				build_expected_str(&None, &foreground, id, "Foreground NONE")
			);

			let id = ID_SEED.fetch_add(1, Ordering::Relaxed);
			let act = build_string_str(
				&foreground,
				&None,
				&format!("<<contain>>{}{}", id, "Background NONE"),
			);
			assert_eq!(
				act,
				build_expected_str(&foreground, &None, id, "Background NONE")
			);
		}

		let act = build_string_str(&None, &None, "<<contain>>42NONE");
		assert_eq!(act, build_expected_str(&None, &None, 42, "NONE"));
	}

	#[test]
	fn build_string_args_test() {
		for foreground in COLORS.iter().map(Some) {
			for background in COLORS.iter().map(Some) {
				let id = ID_SEED.fetch_add(1, Relaxed);
				assert_eq!(
					build_string_args(
						&foreground,
						&background,
						format_args!("<<contain>>{}{}", id, "ARGS")
					),
					build_expected_str(&foreground, &background, id, "ARGS")
				);
			}

			let id = ID_SEED.fetch_add(1, Relaxed);
			assert_eq!(
				build_string_args(
					&foreground,
					&None,
					format_args!("<<contain>>{}{}", id, "BackgroundNone")
				),
				build_expected_str(&foreground, &None, id, "BackgroundNone")
			);

			let id = ID_SEED.fetch_add(1, Relaxed);
			assert_eq!(
				build_string_args(
					&None,
					&foreground,
					format_args!("<<contain>>{}{}", id, "ForegroundNone")
				),
				build_expected_str(&None, &foreground, id, "ForegroundNone")
			);
		}
	}

	#[test]
	fn println_test() {
		for fore in COLORS.iter().map(Some) {
			for back in COLORS.iter().map(Some) {
				let id = ID_SEED.fetch_add(1, Relaxed);
				println_str(&fore, &back, &format!("<<contain>>{}{}", id, "BOTH"));
				assert_ln_output(&build_expected_str(&fore, &back, id, "BOTH"));
			}

			let id = ID_SEED.fetch_add(1, Relaxed);
			println_str(&fore, &None, &format!("<<contain>>{}{}", id, "BackNone"));
			assert_ln_output(&build_expected_str(&fore, &None, id, "BackNone"));

			let id = ID_SEED.fetch_add(1, Relaxed);
			println_str(&None, &fore, &format!("<<contain>>{}{}", id, "ForeNone"));
			assert_ln_output(&build_expected_str(&None, &fore, id, "ForeNone"));
		}
	}

	#[test]
	fn println_args_test() {
		for fore in COLORS.iter().map(Some) {
			for back in COLORS.iter().map(Some) {
				let id = ID_SEED.fetch_add(1, Relaxed);
				println_args(
					&fore,
					&back,
					format_args!("<<contain>>{}{}", id, "ARG_BOTH"),
				);
				assert_ln_output(&build_expected_str(&fore, &back, id, "ARG_BOTH"));
			}

			let id = ID_SEED.fetch_add(1, Relaxed);
			println_args(
				&fore,
				&None,
				format_args!("<<contain>>{}{}", id, "ARG_BACK_NONE"),
			);
			assert_ln_output(&build_expected_str(&fore, &None, id, "ARG_BACK_NONE"));

			let id = ID_SEED.fetch_add(1, Relaxed);
			println_args(
				&None,
				&fore,
				format_args!("<<contain>>{}{}", id, "ARG_FORE_NONE"),
			);
			assert_ln_output(&build_expected_str(&None, &fore, id, "ARG_FORE_NONE"));
		}
	}

	#[test]
	fn print_str_test() {
		for (fore, back) in all_combination() {
			let id = ID_SEED.fetch_add(1, Relaxed);
			print_str(
				&Some(fore),
				&Some(back),
				&format!("<<contain>>{}{}", id, "BOTH"),
			);
			assert_output(&build_expected_str(&Some(fore), &Some(back), id, "BOTH"))
		}

		for col in COLORS.iter() {
			let id = ID_SEED.fetch_add(1, Relaxed);
			print_str(&Some(col), &None, &format!("<<contain>>{}{}", id, "FORE"));
			assert_output(&build_expected_str(&Some(col), &None, id, "FORE"));

			let id = ID_SEED.fetch_add(1, Relaxed);
			print_str(&Some(col), &None, &format!("<<contain>>{}{}", id, "BACK"));
			assert_output(&build_expected_str(&Some(col), &None, id, "BACK"));
		}

		let id = ID_SEED.fetch_add(1, Relaxed);
		print_str(&None, &None, &format!("<<contain>>{}{}", id, "NONE"));
		assert_output(&build_expected_str(&None, &None, id, "NONE"));
	}

	#[test]
	fn print_arg_test() {
		for (fore, back) in all_combination() {
			let id = ID_SEED.fetch_add(1, Relaxed);

			print_args(
				&Some(&fore),
				&Some(&back),
				format_args!("<<contain>>{}{}", id, "BOTH"),
			);
			assert_output(&build_expected_str(&Some(&fore), &Some(&back), id, "BOTH"));
		}

		for col in COLORS.iter() {
			let id = ID_SEED.fetch_add(1, Relaxed);
			print_args(&Some(&col), &None, format_args!("<<contain>>{}FORE", id));
			assert_output(&build_expected_str(&Some(&col), &None, id, "FORE"));
		}

		let id = ID_SEED.fetch_add(1, Relaxed);
		print_args(&None, &None, format_args!("<<contain>>{}{}", id, "NONE"));
		assert_output(&build_expected_str(&None, &None, id, "NONE"));
	}
}
