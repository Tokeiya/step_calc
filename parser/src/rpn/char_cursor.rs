pub struct CharCursor {
	vec: Vec<char>,
	pos: usize,
}

impl CharCursor {
	pub fn next(&mut self) -> Option<char> {
		if (self.vec.len() <= self.pos) {
			None
		} else {
			let ret = self.vec[self.pos];
			self.pos += 1;
			Some(ret)
		}
	}

	pub fn previous(&mut self) -> Option<char> {
		if self.pos == 0 {
			None
		} else {
			self.pos -= 1;
			Some(self.vec[self.pos])
		}
	}

	pub fn remainder_to_string(&self) -> String {
		todo!()
	}

	pub fn consumed_to_string(&self) -> String {
		todo!()
	}
}

impl From<&str> for CharCursor {
	fn from(value: &str) -> Self {
		CharCursor {
			vec: value.chars().collect(),
			pos: 0,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const SAMPLE: &str = "abcdefghijklmnopqrstuvwxyz";

	#[test]
	fn cursor_init_test() {
		let fixture = CharCursor::from(SAMPLE);
		assert_eq!(fixture.pos, 0);

		let expected: Vec<_> = SAMPLE.chars().collect();

		assert_eq!(fixture.vec.len(), expected.len());

		for (idx, (act, exp)) in fixture.vec.iter().zip(expected.iter()).enumerate() {
			assert_eq!(act, exp, "{idx} mismatch")
		}
	}

	#[test]
	fn next_test() {
		let mut fixture = CharCursor::from(SAMPLE);
		let expected: Vec<_> = SAMPLE.chars().collect();

		for e in expected.iter() {
			let a = fixture.next().unwrap();
			assert_eq!(&a, e);
		}

		for _ in 0..10 {
			assert_eq!(fixture.pos, expected.len());
			assert!(fixture.next().is_none());
		}
	}

	#[test]
	fn previous_test() {
		let mut fixture = CharCursor::from(SAMPLE);
		let expected: Vec<_> = SAMPLE.chars().rev().collect();

		while fixture.next().is_some() {}

		for e in expected.iter() {
			let a = fixture.previous().unwrap();
			assert_eq!(&a, e)
		}

		for _ in 0..10 {
			assert_eq!(fixture.pos, 0);
			assert!(fixture.previous().is_none());
		}
	}

	#[test]
	fn remainder_to_string_test() {
		let mut fixture = CharCursor::from(SAMPLE);

		for i in 0..SAMPLE.len() {
			let actual = fixture.remainder_to_string();
			assert_eq!(&actual, &SAMPLE[i..]);

			let mut consumed = fixture.consumed_to_string();
			consumed.push_str(&actual);
			assert_eq!(&consumed, SAMPLE)
		}

		for _ in 0..10 {
			let actual = fixture.remainder_to_string();
			assert_eq!(&actual, "");
		}

		for i in (0..SAMPLE.len()).rev() {
			let actual = fixture.remainder_to_string();
			assert_eq!(&actual, &SAMPLE[i..]);

			let mut consumed = fixture.consumed_to_string();
			consumed.push_str(&actual);
			assert_eq!(&consumed, SAMPLE);
		}
	}

	#[test]
	fn consumed_to_string_test() {
		let mut fixture = CharCursor::from(SAMPLE);

		for i in 0..SAMPLE.len() {
			let exp = &SAMPLE[..i];
			let mut consumed = fixture.consumed_to_string();
			assert_eq!(&consumed, exp);

			consumed.push_str(&fixture.remainder_to_string());
			assert_eq!(&consumed, SAMPLE)
		}

		for _ in 0..10 {
			let actual = fixture.consumed_to_string();
			assert_eq!(&actual, SAMPLE)
		}

		for i in (0..SAMPLE.len()).rev() {
			let exp = &SAMPLE[..i];
			let mut consumed = fixture.consumed_to_string();
			assert_eq!(&consumed, exp);

			consumed.push_str(&fixture.remainder_to_string());
			assert_eq!(&consumed, SAMPLE)
		}
	}
}
