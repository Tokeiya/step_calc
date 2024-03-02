#[cfg(test)]
#[allow(dead_code)]
pub enum TrimOption {
	End,
	Start,
	Both,
}

fn str_to_vec(scr: &str, ignore_whitespace: bool) -> Vec<&str> {
	let mut vec = Vec::<&str>::default();

	for line in scr.lines() {
		if ignore_whitespace && line.trim().is_empty() {
			continue;
		}

		vec.push(line)
	}

	vec
}

#[cfg(test)]
fn assert_text(actual: &str, expected: &str, trim: Option<&[TrimOption]>, ignore_whitespace: bool) {
	let a = str_to_vec(actual, ignore_whitespace);
	let e = str_to_vec(expected, ignore_whitespace);

	assert_eq!(a.len(), e.len());

	for (idx, exp, act) in e
		.iter()
		.enumerate()
		.zip(e.iter())
		.map(|(x, y)| (x.0, x.1, y))
	{
		let mut a = *act;
		let mut e = *exp;

		if let Some(tarry) = trim {
			for t in tarry {
				match t {
					TrimOption::End => {
						a = a.trim_end();
						e = e.trim_end();
					}
					TrimOption::Start => {
						a = a.trim_start();
						e = e.trim_start();
					}
					TrimOption::Both => {
						a = a.trim();
						e = e.trim();
					}
				}
			}
		}
		assert_eq!(act, exp, "{} exp:{} act:{}", idx, e, a);
	}
}

#[cfg(test)]
pub fn strict_assert_text(actual: &str, expected: &str) {
	assert_text(actual, expected, None, false)
}

#[cfg(test)]
pub fn trimed_assert_text(actual: &str, expected: &str) {
	assert_text(actual, expected, Some(&[TrimOption::Both]), false)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn str_to_ve_test() {
		const SAMPLE: &str = r##"a
b

c
	
d
   
   e   
"##;
		let fixture = str_to_vec(SAMPLE, false);
		assert_eq!(fixture.len(), 8);
		assert_eq!(fixture[0], "a");
		assert_eq!(fixture[1], "b");
		assert_eq!(fixture[2], "");
		assert_eq!(fixture[3], "c");
		assert_eq!(fixture[4], "\t");
		assert_eq!(fixture[5], "d");
		assert_eq!(fixture[6], "   ");
		assert_eq!(fixture[7], "   e   ");

		let fixture = str_to_vec(SAMPLE, true);
		assert_eq!(fixture.len(), 5);

		assert_eq!(fixture[0], "a");
		assert_eq!(fixture[1], "b");
		assert_eq!(fixture[2], "c");
		assert_eq!(fixture[3], "d");
		assert_eq!(fixture[4], "   e   ");
	}
}
