pub const RESET: u8 = 0;

pub enum ColorContext {
	Foreground,
	Background,
}

pub enum ConsoleColor {
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
	Gray,
	BrightRed,
	BrightGreen,
	BrightYellow,
	BrightBlue,
	BrightMagenta,
	BrightCyan,
	BrightWhite,
}

impl ConsoleColor {
	pub fn foreground(&self) -> u8 {
		match self {
			ConsoleColor::Black => 30,
			ConsoleColor::Red => 31,
			ConsoleColor::Green => 32,
			ConsoleColor::Yellow => 33,
			ConsoleColor::Blue => 34,
			ConsoleColor::Magenta => 35,
			ConsoleColor::Cyan => 36,
			ConsoleColor::White => 37,
			ConsoleColor::Gray => 90,
			ConsoleColor::BrightRed => 91,
			ConsoleColor::BrightGreen => 92,
			ConsoleColor::BrightYellow => 93,
			ConsoleColor::BrightBlue => 94,
			ConsoleColor::BrightMagenta => 95,
			ConsoleColor::BrightCyan => 96,
			ConsoleColor::BrightWhite => 97,
		}
	}

	pub fn background(&self) -> u8 {
		match self {
			ConsoleColor::Black => 40,
			ConsoleColor::Red => 41,
			ConsoleColor::Green => 42,
			ConsoleColor::Yellow => 43,
			ConsoleColor::Blue => 44,
			ConsoleColor::Magenta => 45,
			ConsoleColor::Cyan => 46,
			ConsoleColor::White => 47,
			ConsoleColor::Gray => 100,
			ConsoleColor::BrightRed => 101,
			ConsoleColor::BrightGreen => 102,
			ConsoleColor::BrightYellow => 103,
			ConsoleColor::BrightBlue => 104,
			ConsoleColor::BrightMagenta => 105,
			ConsoleColor::BrightCyan => 106,
			ConsoleColor::BrightWhite => 107,
		}
	}

	pub fn color(&self, context: &ColorContext) -> u8 {
		match context {
			ColorContext::Foreground => self.foreground(),
			ColorContext::Background => self.background(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn foreground_test() {
		assert_eq!(ConsoleColor::Black.foreground(), 30);
		assert_eq!(ConsoleColor::Red.foreground(), 31);
		assert_eq!(ConsoleColor::Green.foreground(), 32);
		assert_eq!(ConsoleColor::Yellow.foreground(), 33);
		assert_eq!(ConsoleColor::Blue.foreground(), 34);
		assert_eq!(ConsoleColor::Magenta.foreground(), 35);
		assert_eq!(ConsoleColor::Cyan.foreground(), 36);
		assert_eq!(ConsoleColor::White.foreground(), 37);
		assert_eq!(ConsoleColor::Gray.foreground(), 90);
		assert_eq!(ConsoleColor::BrightRed.foreground(), 91);
		assert_eq!(ConsoleColor::BrightGreen.foreground(), 92);
		assert_eq!(ConsoleColor::BrightYellow.foreground(), 93);
		assert_eq!(ConsoleColor::BrightBlue.foreground(), 94);
		assert_eq!(ConsoleColor::BrightMagenta.foreground(), 95);
		assert_eq!(ConsoleColor::BrightCyan.foreground(), 96);
		assert_eq!(ConsoleColor::BrightWhite.foreground(), 97);
	}

	#[test]
	fn background_test() {
		assert_eq!(ConsoleColor::Black.background(), 40);
		assert_eq!(ConsoleColor::Red.background(), 41);
		assert_eq!(ConsoleColor::Green.background(), 42);
		assert_eq!(ConsoleColor::Yellow.background(), 43);
		assert_eq!(ConsoleColor::Blue.background(), 44);
		assert_eq!(ConsoleColor::Magenta.background(), 45);
		assert_eq!(ConsoleColor::Cyan.background(), 46);
		assert_eq!(ConsoleColor::White.background(), 47);
		assert_eq!(ConsoleColor::Gray.background(), 100);
		assert_eq!(ConsoleColor::BrightRed.background(), 101);
		assert_eq!(ConsoleColor::BrightGreen.background(), 102);
		assert_eq!(ConsoleColor::BrightYellow.background(), 103);
		assert_eq!(ConsoleColor::BrightBlue.background(), 104);
		assert_eq!(ConsoleColor::BrightMagenta.background(), 105);
		assert_eq!(ConsoleColor::BrightCyan.background(), 106);
		assert_eq!(ConsoleColor::BrightWhite.background(), 107);
	}

	#[test]
	fn color_test() {
		assert_eq!(ConsoleColor::Black.color(&ColorContext::Foreground), 30);
		assert_eq!(ConsoleColor::Red.color(&ColorContext::Foreground), 31);
		assert_eq!(ConsoleColor::Green.color(&ColorContext::Foreground), 32);
		assert_eq!(ConsoleColor::Yellow.color(&ColorContext::Foreground), 33);
		assert_eq!(ConsoleColor::Blue.color(&ColorContext::Foreground), 34);
		assert_eq!(ConsoleColor::Magenta.color(&ColorContext::Foreground), 35);
		assert_eq!(ConsoleColor::Cyan.color(&ColorContext::Foreground), 36);
		assert_eq!(ConsoleColor::White.color(&ColorContext::Foreground), 37);
		assert_eq!(ConsoleColor::Gray.color(&ColorContext::Foreground), 90);
		assert_eq!(ConsoleColor::BrightRed.color(&ColorContext::Foreground), 91);
		assert_eq!(
			ConsoleColor::BrightGreen.color(&ColorContext::Foreground),
			92
		);
		assert_eq!(
			ConsoleColor::BrightYellow.color(&ColorContext::Foreground),
			93
		);
		assert_eq!(
			ConsoleColor::BrightBlue.color(&ColorContext::Foreground),
			94
		);
		assert_eq!(
			ConsoleColor::BrightMagenta.color(&ColorContext::Foreground),
			95
		);
		assert_eq!(
			ConsoleColor::BrightCyan.color(&ColorContext::Foreground),
			96
		);
		assert_eq!(ConsoleColor::Black.color(&ColorContext::Background), 40);
		assert_eq!(ConsoleColor::Red.color(&ColorContext::Background), 41);
		assert_eq!(ConsoleColor::Green.color(&ColorContext::Background), 42);
		assert_eq!(ConsoleColor::Yellow.color(&ColorContext::Background), 43);
		assert_eq!(ConsoleColor::Blue.color(&ColorContext::Background), 44);
		assert_eq!(ConsoleColor::Magenta.color(&ColorContext::Background), 45);
		assert_eq!(ConsoleColor::Cyan.color(&ColorContext::Background), 46);
		assert_eq!(ConsoleColor::White.color(&ColorContext::Background), 47);
		assert_eq!(ConsoleColor::Gray.color(&ColorContext::Background), 100);
		assert_eq!(
			ConsoleColor::BrightRed.color(&ColorContext::Background),
			101
		);
		assert_eq!(
			ConsoleColor::BrightGreen.color(&ColorContext::Background),
			102
		);
		assert_eq!(
			ConsoleColor::BrightYellow.color(&ColorContext::Background),
			103
		);
		assert_eq!(
			ConsoleColor::BrightBlue.color(&ColorContext::Background),
			104
		);
		assert_eq!(
			ConsoleColor::BrightMagenta.color(&ColorContext::Background),
			105
		);
		assert_eq!(
			ConsoleColor::BrightCyan.color(&ColorContext::Background),
			106
		);
		assert_eq!(
			ConsoleColor::BrightWhite.color(&ColorContext::Background),
			107
		);
	}

	#[test]
	fn reset_test() {
		assert_eq!(RESET, 0);
	}
}
