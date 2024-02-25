use std::io::{Read, Result as IoResult, Write};
use std::ops::Index;
use std::process::{Command, Stdio};

use anyhow::Result as AnyResult;

pub fn generate_svg(scr: &str) -> IoResult<String> {
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

pub fn extract_svg_element(scr: &str) -> AnyResult<String> {
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
	writer.write(br"<!DOCTYPE html>")?;
	writer.write(b"\n")?;

	writer.write(br#"<html lang="ja">"#)?;
	writer.write(b"\n")?;

	writer.write(br"<head>")?;
	writer.write(b"\n")?;

	writer.write(br#"<meta charset="UTF-8">"#)?;
	writer.write(b"\n")?;

	writer.write_fmt(format_args!(r#"<title>{}</title>"#, formula))?;
	writer.write(b"\n")?;

	writer.write(br"</head>")?;
	writer.write(b"\n")?;

	writer.write(br"<body>")?;
	writer.write(b"\n")?;

	writer.write_fmt(format_args!(r#"<h1>{}</h1>"#, formula))?;
	Ok(())
}

pub fn write_html(formula: &str, writer: &mut dyn Write) -> IoResult<()> {
	todo!()
}

fn write_footer() -> IoResult<()> {
	todo!()
}

pub fn create_document(title: &str, svg: &str, path: &str) {}

#[cfg(test)]
pub mod tests {
	use std::fs::File;
	use std::io::{Cursor, Read};

	use once_cell::sync::Lazy;

	use super::*;

	const EXPECTED_DOT: Lazy<String> = Lazy::new(|| {
		let mut file = File::open("./test_artifacts/sample.dot").unwrap();
		let mut str = String::default();

		file.read_to_string(&mut str).unwrap();
		str
	});

	const EXPECTED_FULL_SVG: Lazy<String> = Lazy::new(|| {
		let mut file = File::open("./test_artifacts/sample.svg").unwrap();
		let mut str = String::default();

		file.read_to_string(&mut str).unwrap();
		str
	});

	const EXPECTED_HTML: Lazy<String> = Lazy::new(|| {
		let mut file = File::open("./test_artifacts/sample.txt").unwrap();
		let mut str = String::default();

		file.read_to_string(&mut str).unwrap();
		str
	});

	const SAMPLE_FORMULA: &str = "{1+2*3}/{{4-5}*{{6+7}/2}}";

	fn create_cursor() -> Cursor<Vec<u8>> {
		Cursor::<Vec<u8>>::default()
	}

	#[test]
	fn extract() {
		let reg = regex::Regex::new(r"(?s)<svg.*?</svg>").unwrap();

		let binding = EXPECTED_FULL_SVG;
		let binding = reg.captures(binding.as_str()).unwrap();
		let expected = binding.index(0);
	}

	#[test]
	fn header() {
		let reg = regex::Regex::new(r"(?s)^<!DOCTYPE.*?</h1>").unwrap();
		let binding = EXPECTED_HTML;
		let cap = reg.captures(binding.as_str());
		let expected = cap.unwrap().index(0).to_string();

		let mut cursor = create_cursor();
		write_header(SAMPLE_FORMULA, &mut cursor).unwrap();
		let act = String::from_utf8(cursor.into_inner()).unwrap();

		assert_eq!(act, expected)
	}
}
