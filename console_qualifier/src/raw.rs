use std::fmt::Arguments;

use super::color::{ColorContext, ConsoleColor};

const PREFIX: &str = r"\x1B[";
const POSTFIX: &str = "m";

pub fn build_escape(color: Option<&ConsoleColor>, context: &ColorContext) -> String {
	match color {
		None => String::default(),
		Some(col) => match context {
			ColorContext::Foreground => format!(r"\x1B[{}m", col.foreground()),
			ColorContext::Background => format!(r"\x1B[{}m", col.background()),
		},
	}
}

pub fn reset() -> &'static str {
	r"\x1B[0m"
}

pub fn str_build_string(
	foreground: Option<&ConsoleColor>,
	background: Option<&ConsoleColor>,
	content: &str,
) -> String {
	todo!()
}

pub fn arg_build_string(
	foreground: Option<&ConsoleColor>,
	background: Option<&ConsoleColor>,
	arg: Arguments,
) -> String {
	todo!()
}

pub fn add_color(
	foreground: Option<&ConsoleColor>,
	background: Option<&ConsoleColor>,
	mut text: String,
) {
	todo!()
}

pub fn str_println(
	foreground: Option<&ConsoleColor>,
	background: Option<&ConsoleColor>,
	content: &str,
) {
	todo!()
}

pub fn args_println(
	foreground: Option<&ConsoleColor>,
	background: Option<&ConsoleColor>,
	args: Arguments<'_>,
) {
	todo!()
}

pub fn str_print(
	foreground: Option<&ConsoleColor>,
	background: Option<&ConsoleColor>,
	content: &str,
) {
	todo!()
}

pub fn args_print(
	foreground: Option<&ConsoleColor>,
	background: Option<&ConsoleColor>,
	args: Arguments<'_>,
) {
	todo!()
}

#[cfg(test)]
mod tests {
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

	#[test]
	fn build_escape_test() {
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Black), &ColorContext::Foreground),
			r"\x1B[30m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Red), &ColorContext::Foreground),
			r"\x1B[31m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Green), &ColorContext::Foreground),
			r"\x1B[32m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Yellow), &ColorContext::Foreground),
			r"\x1B[33m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Blue), &ColorContext::Foreground),
			r"\x1B[34m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Magenta), &ColorContext::Foreground),
			r"\x1B[35m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Cyan), &ColorContext::Foreground),
			r"\x1B[36m"
		);

		assert_eq!(
			super::build_escape(Some(&ConsoleColor::White), &ColorContext::Foreground),
			r"\x1B[37m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Gray), &ColorContext::Foreground),
			r"\x1B[90m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::BrightRed), &ColorContext::Foreground),
			r"\x1B[91m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::BrightGreen), &ColorContext::Foreground),
			r"\x1B[92m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::BrightYellow), &ColorContext::Foreground),
			r"\x1B[93m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::BrightBlue), &ColorContext::Foreground),
			r"\x1B[94m"
		);
		assert_eq!(
			super::build_escape(
				Some(&ConsoleColor::BrightMagenta),
				&ColorContext::Foreground
			),
			r"\x1B[95m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::BrightCyan), &ColorContext::Foreground),
			r"\x1B[96m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::BrightWhite), &ColorContext::Foreground),
			r"\x1B[97m"
		);

		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Black), &ColorContext::Background),
			r"\x1B[40m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Red), &ColorContext::Background),
			r"\x1B[41m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Green), &ColorContext::Background),
			r"\x1B[42m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Yellow), &ColorContext::Background),
			r"\x1B[43m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Blue), &ColorContext::Background),
			r"\x1B[44m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Magenta), &ColorContext::Background),
			r"\x1B[45m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Cyan), &ColorContext::Background),
			r"\x1B[46m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::White), &ColorContext::Background),
			r"\x1B[47m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::Gray), &ColorContext::Background),
			r"\x1B[100m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::BrightRed), &ColorContext::Background),
			r"\x1B[101m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::BrightGreen), &ColorContext::Background),
			r"\x1B[102m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::BrightYellow), &ColorContext::Background),
			r"\x1B[103m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::BrightBlue), &ColorContext::Background),
			r"\x1B[104m"
		);
		assert_eq!(
			super::build_escape(
				Some(&ConsoleColor::BrightMagenta),
				&ColorContext::Background
			),
			r"\x1B[105m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::BrightCyan), &ColorContext::Background),
			r"\x1B[106m"
		);
		assert_eq!(
			super::build_escape(Some(&ConsoleColor::BrightWhite), &ColorContext::Background),
			r"\x1B[107m"
		);
	}

	fn build_expected(
		foreground: Option<&ConsoleColor>,
		background: Option<&ConsoleColor>,
		text: &str,
	) -> String {
		format!(
			r"{}{}{}\x1B[0m",
			build_escape(foreground, &ColorContext::Foreground),
			build_escape(background, &ColorContext::Background),
			text
		)
	}

	#[test]
	fn str_build_string_test() {
		const CONTENT: &str = "Hello, World!";

		for foreground in COLORS.iter().map(Some) {
			for background in COLORS.iter().map(Some) {
				assert_eq!(
					str_build_string(foreground, background, CONTENT),
					format!(
						r"{}{}Hello, World!\x1B[0m",
						build_escape(foreground, &ColorContext::Foreground),
						build_escape(background, &ColorContext::Background)
					)
				);
			}
		}

		for color in COLORS.iter().map(Some) {
			assert_eq!(
				str_build_string(color, None, CONTENT),
				format!(
					r"{}Hello, World!\x1B[0m",
					build_escape(color, &ColorContext::Foreground)
				)
			);

			assert_eq!(
				str_build_string(None, color, CONTENT),
				format!(
					r"{}Hello, World!\x1B[0m",
					build_escape(color, &ColorContext::Background)
				)
			);
		}

		assert_eq!(str_build_string(None, None, CONTENT), "Hello, World");
	}

	#[test]
	fn arg_build_string_test() {
		for fg in COLORS.iter().map(Some) {
			for bg in COLORS.iter().map(Some) {
				assert_eq!(
					arg_build_string(fg, bg, format_args!("hello world")),
					build_expected(fg, bg, "hello world")
				)
			}
		}

		for col in COLORS.iter().map(Some) {
			assert_eq!(
				arg_build_string(col, None, format_args!("hello world")),
				build_expected(col, None, "hello world")
			);
			assert_eq!(
				arg_build_string(None, col, format_args!("hello world")),
				build_expected(None, col, "hello world")
			);
		}

		assert_eq!(
			arg_build_string(None, None, format_args!("hello world")),
			"hello world"
		);
	}
}
