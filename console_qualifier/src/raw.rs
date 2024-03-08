use std::fmt::Arguments;

#[cfg(test)]
use dashmap::DashSet;
#[cfg(test)]
use once_cell::sync::Lazy;

use super::color::ConsoleColor;

pub const PREFIX: &str = r"\x1B[";
pub const POSTFIX: &str = "m";
pub const RESET: &str = "\x1B[0m";

#[cfg(test)]
static ACTUAL: Lazy<DashSet<String>> = Lazy::new(DashSet::default);

#[cfg(test)]
macro_rules! println {
    () => {
        unreachable!();
    };
    ($($arg:tt)*) => ({
        let txt = format!($($arg)*);
        std::println!("{}", &txt);

        if !ACTUAL.insert(txt) {
            panic!("Already inserted");
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

        if !ACTUAL.insert(txt) {
            panic!("Already inserted");
        }
    });
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
		foreground: Option<&ConsoleColor>,
		background: Option<&ConsoleColor>,
		id: usize,
		text: &str,
	) -> String {
		format!(
			"{}{}{}\x1B[0m",
			build_escape(&foreground, &background),
			id,
			text
		)
	}

	fn concat_id(id: usize, txt: &str) -> String {
		format!("{}{}", id, txt)
	}

	fn assert_output(expected: &str) {
		assert!(ACTUAL.remove(expected).is_some())
	}

	#[test]
	fn consistency_test() {
		println!("{}", "foo");
		assert_output("foo");

		print!("{}", "bar");
		assert_output("bar");
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
					&format!("{}{}", id, "hello world"),
				);

				assert_eq!(
					act,
					build_expected_str(foreground, background, id, "hello world")
				);
			}

			let id = ID_SEED.fetch_add(1, Ordering::Relaxed);
			let act = build_string_str(&None, &foreground, &format!("{}{}", id, "Foreground NONE"));
			assert_eq!(
				act,
				build_expected_str(None, foreground, id, "Foreground NONE")
			);

			let id = ID_SEED.fetch_add(1, Ordering::Relaxed);
			let act = build_string_str(&foreground, &None, &format!("{}{}", id, "Background NONE"));
			assert_eq!(
				act,
				build_expected_str(foreground, None, id, "Background NONE")
			);
		}

		let act = build_string_str(&None, &None, "42NONE");
		assert_eq!(act, build_expected_str(None, None, 42, "NONE"));
	}

	#[test]
	fn build_string_args_test() {
		for foreground in COLORS.iter().map(Some) {
			for background in COLORS.iter().map(Some) {
				let id = ID_SEED.fetch_add(1, Relaxed);
				assert_eq!(
					build_string_args(&foreground, &background, format_args!("{}{}", id, "ARGS")),
					build_expected_str(foreground, background, id, "ARGS")
				);
			}

			let id = ID_SEED.fetch_add(1, Relaxed);
			assert_eq!(
				build_string_args(
					&foreground,
					&None,
					format_args!("{}{}", id, "BackgroundNone")
				),
				build_expected_str(foreground, None, id, "BackgroundNone")
			);

			let id = ID_SEED.fetch_add(1, Relaxed);
			assert_eq!(
				build_string_args(
					&None,
					&foreground,
					format_args!("{}{}", id, "ForegroundNone")
				),
				build_expected_str(None, foreground, id, "ForegroundNone")
			);
		}
	}

	#[test]
	fn println_test() {
		for fore in COLORS.iter().map(Some) {
			for back in COLORS.iter().map(Some) {
				let id = ID_SEED.fetch_add(1, Relaxed);
				println_str(&fore, &back, &format!("{}{}", id, "BOTH"));
				assert_output(&build_expected_str(fore, back, id, "BOTH"));
			}

			let id = ID_SEED.fetch_add(1, Relaxed);
			println_str(&fore, &None, &format!("{}{}", id, "BackNone"));
			assert_output(&build_expected_str(fore, None, id, "BackNone"));

			let id = ID_SEED.fetch_add(1, Relaxed);
			println_str(&None, &fore, &format!("{}{}", id, "ForeNone"));
			assert_output(&build_expected_str(None, fore, id, "ForeNone"));
		}
	}

	#[test]
	fn println_args_test() {
		for fore in COLORS.iter().map(Some) {
			for back in COLORS.iter().map(Some) {
				let id = ID_SEED.fetch_add(1, Relaxed);
				println_args(&fore, &back, format_args!("{}{}", id, "ARG_BOTH"));
				assert_output(&build_expected_str(fore, back, id, "ARG_BOTH"));
			}

			let id = ID_SEED.fetch_add(1, Relaxed);
			println_args(&fore, &None, format_args!("{}{}", id, "ARG_BACK_NONE"));
			assert_output(&build_expected_str(fore, None, id, "ARG_BACK_NONE"));

			let id = ID_SEED.fetch_add(1, Relaxed);
			println_args(&None, &fore, format_args!("{}{}", id, "ARG_FORE_NONE"));
			assert_output(&build_expected_str(None, fore, id, "ARG_FORE_NONE"));
		}
	}
}
