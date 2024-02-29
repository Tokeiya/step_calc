pub struct CharCursor {
	vec: Vec<char>,
	pos: usize,
}

pub struct CursorIterator<'a> {
	slice: &'a [char],
	idx: usize,
}

impl<'a> CursorIterator<'a> {
	fn new(scr: &'a [char]) -> Self {
		CursorIterator { slice: scr, idx: 0 }
	}
}

impl<'a> Iterator for CursorIterator<'a> {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		if (self.idx >= self.slice.len()) {
			None
		} else {
			let c = self.slice[self.idx];
			self.idx += 1;
			Some(c)
		}
	}
}

impl CharCursor {
	pub fn iter(&self) -> CursorIterator<'_> {
		CursorIterator::new(&self.vec[self.pos..])
	}

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
		todo!()
	}
}

impl From<&str> for CharCursor {
	fn from(value: &str) -> Self {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn cursor_init() {
		todo!()
	}
}
