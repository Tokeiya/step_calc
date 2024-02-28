use std::io::{Cursor, Read, Result as IoResult, Write};
use std::ops::Index;
use std::process::{Command, Stdio};

use anyhow::Result as AnyResult;

use parser::infix::parser::parse;
use syntax::dot_writer::write_dot;

fn generate_svg(scr: &str) -> IoResult<String> {
	let mut proc = Command::new("dot")
		.args(&["-Tsvg"])
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()?;

	if let Some(mut stdin) = proc.stdin.take() {
		stdin.write_all(scr.as_bytes())?;
	}

	let mut buff = String::default();
	if let Some(mut stdout) = proc.stdout.take() {
		stdout.read_to_string(&mut buff)?;
	}

	Ok(buff)
}

fn extract_svg_element(scr: &str) -> AnyResult<String> {
	let reg = regex::Regex::new(r"(?s)<svg.*?</svg>")?;

	return if let Some(cap) = reg.captures(scr) {
		if cap.len() == 1 {
			Ok(cap.index(0).to_string())
		} else {
			Err(anyhow::Error::msg(format!(
				"Expected len=1 but {}",
				cap.len()
			)))
		}
	} else {
		Err(anyhow::Error::msg("No captured."))
	};
}

fn write_header(formula: &str, writer: &mut dyn Write) -> IoResult<()> {
	_ = writer.write(br"<!DOCTYPE html>")?;
	_ = writer.write(b"\n")?;

	_ = writer.write(br#"<html lang="ja">"#)?;
	_ = writer.write(b"\n")?;

	_ = writer.write(br"<head>")?;
	_ = writer.write(b"\n")?;

	_ = writer.write(br#"<meta charset="UTF-8">"#)?;
	_ = writer.write(b"\n")?;

	writer.write_fmt(format_args!(r#"<title>{formula}</title>"#))?;
	_ = writer.write(b"\n")?;

	_ = writer.write(br"</head>")?;
	_ = writer.write(b"\n")?;

	_ = writer.write(br"<body>")?;
	_ = writer.write(b"\n")?;

	writer.write_fmt(format_args!(r#"<h1>{}</h1>"#, formula))
}

fn write_single_infix_html(formula: &str, writer: &mut dyn Write) -> AnyResult<()> {
	write_header(formula, writer)?;

	let tree = parse(formula)?.0;
	let mut cursor = Cursor::<Vec<u8>>::default();

	write_dot(&mut cursor, &tree)?;
	let dot = String::from_utf8(cursor.into_inner())?;

	let svg = generate_svg(&dot)?;

	let svg = extract_svg_element(&svg)?;

	_ = writer.write(svg.as_bytes())?;

	write_footer(writer)?;

	Ok(())
}

fn write_footer(writer: &mut dyn Write) -> IoResult<()> {
	_ = writer.write(br"</body>")?;
	_ = writer.write(b"\n")?;

	_ = writer.write(br"</html>")?;

	Ok(())
}

#[cfg(test)]
pub mod tests {
	use std::fs::File;

	use once_cell::sync::Lazy;

	use super::*;

	static EXPECTED_FULL_SVG: Lazy<String> = Lazy::new(|| {
		let mut file = File::open("./test_artifacts/sample.txt").unwrap();
		let mut str = String::default();

		file.read_to_string(&mut str).unwrap();
		str
	});

	static EXPECTED_HTML: Lazy<String> = Lazy::new(|| {
		let mut file = File::open("./test_artifacts/full_html.txt").unwrap();
		let mut str = String::default();

		file.read_to_string(&mut str).unwrap();
		str
	});

	const SAMPLE_FORMULA: &str = "{1+2*3}/{{4-5}*{{6+7}/2}}";

	fn create_cursor() -> Cursor<Vec<u8>> {
		Cursor::<Vec<u8>>::default()
	}

	fn assert_text(actual: &str, expected: &str) {
		let a: Vec<_> = actual.lines().collect();
		let e: Vec<_> = expected.lines().collect();

		assert_eq!(a.len(), e.len());

		for (idx, exp, act) in e
			.iter()
			.enumerate()
			.zip(e.iter())
			.map(|(x, y)| (x.0, x.1, y))
		{
			assert_eq!(act, exp, "{} {} {}", idx, exp, act);
		}
	}

	#[test]
	fn generate() {
		let tree = parse(SAMPLE_FORMULA).unwrap().0;

		let mut cursor = create_cursor();
		write_dot(&mut cursor, &tree).unwrap();

		let dot = String::from_utf8(cursor.into_inner()).unwrap();
		let act = generate_svg(&dot).unwrap();

		assert_text(&act, EXPECTED_FULL_SVG.as_str())
	}

	#[test]
	fn single_infix_html() {
		let mut cursor = create_cursor();
		write_single_infix_html(SAMPLE_FORMULA, &mut cursor).unwrap();
		let act = String::from_utf8(cursor.into_inner()).unwrap();

		assert_text(&act, EXPECTED_HTML.as_str())
	}

	#[test]
	fn extract() {
		let reg = regex::Regex::new(r"(?s)<svg.*?</svg>").unwrap();

		let binding = EXPECTED_FULL_SVG.as_str();
		let binding = reg.captures(binding).unwrap();
		let expected = binding.index(0);

		let act = extract_svg_element(EXPECTED_FULL_SVG.as_str()).unwrap();
		assert_text(&act, expected)
	}

	#[test]
	fn header() {
		let reg = regex::Regex::new(r"(?s)^<!DOCTYPE.*?</h1>").unwrap();
		let binding = EXPECTED_HTML.as_str();
		let cap = reg.captures(binding);
		let expected = cap.unwrap().index(0).to_string();

		let mut cursor = create_cursor();
		write_header(SAMPLE_FORMULA, &mut cursor).unwrap();
		let act = String::from_utf8(cursor.into_inner()).unwrap();

		assert_text(&act, &expected);
	}

	#[test]
	fn footer() {
		const EXPECTED: &str = "</body>\n</html>";

		let mut cursor = create_cursor();
		write_footer(&mut cursor).unwrap();

		let act = String::from_utf8(cursor.into_inner()).unwrap();
		assert_text(&act, EXPECTED);
	}
}
