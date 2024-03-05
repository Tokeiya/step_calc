use std::fmt::Arguments;

pub enum ConsoleColor {
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
	BrightBlack,
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
			ConsoleColor::Black => 30u8,
			ConsoleColor::Red => 31u8,
			ConsoleColor::Green => 32u8,
			ConsoleColor::Yellow => 33u8,
			ConsoleColor::Blue => 34u8,
			ConsoleColor::Magenta => 35u8,
			ConsoleColor::Cyan => 36u8,
			ConsoleColor::White => 37u8,
			ConsoleColor::BrightBlack => 90u8,
			ConsoleColor::BrightRed => 91u8,
			ConsoleColor::BrightGreen => 92u8,
			ConsoleColor::BrightYellow => 93u8,
			ConsoleColor::BrightBlue => 94u8,
			ConsoleColor::BrightMagenta => 95u8,
			ConsoleColor::BrightCyan => 96u8,
			ConsoleColor::BrightWhite => 97u8,
		}
	}

	pub fn background(&self) -> u8 {
		match self {
			ConsoleColor::Black => 40u8,
			ConsoleColor::Red => 41u8,
			ConsoleColor::Green => 42u8,
			ConsoleColor::Yellow => 43u8,
			ConsoleColor::Blue => 44u8,
			ConsoleColor::Magenta => 45u8,
			ConsoleColor::Cyan => 46u8,
			ConsoleColor::White => 47u8,
			ConsoleColor::BrightBlack => 100u8,
			ConsoleColor::BrightRed => 101u8,
			ConsoleColor::BrightGreen => 102u8,
			ConsoleColor::BrightYellow => 103u8,
			ConsoleColor::BrightBlue => 104u8,
			ConsoleColor::BrightMagenta => 105u8,
			ConsoleColor::BrightCyan => 106u8,
			ConsoleColor::BrightWhite => 107u8,
		}
	}

	pub fn reset() -> u8 {
		0u8
	}

	pub fn set_foreground(&self, args: Arguments<'_>) -> String {
		format!("{}{}m", 0x1B, self.foreground())
	}

	pub fn add_foreground(&self, str: &mut String) {
		str.push_str(&format!("{}{}m", 0x1B, self.foreground()));
	}

	pub fn reset_style() {
		print!("\x1B[0m");
	}
}
