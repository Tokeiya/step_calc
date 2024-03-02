use std::io::{Cursor, Read, Result as IoResult, Write};
use std::ops::Index;
use std::process::{Command, Stdio};

use anyhow::Result as AnyResult;

use parser::infix::formatter::minimal_infix_notation;
use parser::infix::parser::parse;
use syntax::arithmetic_expression::ArithmeticExpression;
use syntax::dot_writer::write_dot;
use syntax::expression::Expression;

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

pub fn write_single_infix_html(formula: &str, writer: &mut dyn Write) -> AnyResult<()> {
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

	fn write_footer(writer: &mut dyn Write) -> IoResult<()> {
		_ = writer.write(br"</body>")?;
		_ = writer.write(b"\n")?;

		_ = writer.write(br"</html>")?;

		Ok(())
	}

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

fn gen_svg(expr: &Expression) -> AnyResult<String> {
	let mut cursor = Cursor::<Vec<u8>>::default();
	write_dot(&mut cursor, expr)?;

	let txt = String::from_utf8(cursor.into_inner())?;
	let txt = extract_svg_element(&generate_svg(&txt)?)?;

	Ok(txt)
}

fn write_step(recent: Option<&str>, expr: &Expression, writer: &mut dyn Write) -> AnyResult<()> {
	let current_expr = minimal_infix_notation(&expr);
	_ = writer.write(
		br##"<div class="step">
    <h1 class="formula">
"##,
	)?;

	if let Some(recent) = recent {
		writer.write_fmt(format_args!(
			r"{}<br/>
		{}
		",
			recent, current_expr
		))?;
	} else {
		writer.write_fmt(format_args!(
			r"{}
		",
			current_expr
		))?;
	}
	_ = writer.write(b"</h1>")?;

	let svg = gen_svg(&expr)?;
	_ = writer.write(svg.as_bytes())?;

	_ = writer.write(b"</div>")?;

	Ok(())
}

pub fn write_step_infix_html(formula: &str, writer: &mut dyn Write) -> AnyResult<()> {
	writer.write_fmt(format_args!(
		r##"<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="UTF-8">
<title>{}</title>
</head>
<style>
    .step {{
        margin-bottom: 40px; /* 各<div>の下に20pxの余白を追加 */
        border-bottom: 2px solid black; /* 下側にのみ黒の境界線を追加 */
    }}

    .formula{{
        margin-bottom: 10px;
        border-bottom: 5px solid darkgray;
    }}
    
</style>

<body>"##,
		formula
	))?;

	let mut recent = parse(formula)?.0.simplify();
	write_step(None, &recent, writer)?;

	loop {
		let (expr, is_proceed) = recent.step_calc();

		println!("{}", minimal_infix_notation(&expr));

		if !is_proceed {
			break;
		}
		let recent_expr = minimal_infix_notation(&recent);
		write_step(Some(&recent_expr), &expr, writer)?;

		recent = expr;
	}

	_ = writer.write(br"</body></html>")?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use std::fs::File;

	use once_cell::sync::Lazy;

	use crate::test_helper::strict_assert_text;

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

	#[test]
	fn generate() {
		let tree = parse(SAMPLE_FORMULA).unwrap().0;

		let mut cursor = create_cursor();
		write_dot(&mut cursor, &tree).unwrap();

		let dot = String::from_utf8(cursor.into_inner()).unwrap();
		let act = generate_svg(&dot).unwrap();

		strict_assert_text(&act, EXPECTED_FULL_SVG.as_str())
	}

	#[test]
	fn single_infix_html() {
		let mut cursor = create_cursor();
		write_single_infix_html(SAMPLE_FORMULA, &mut cursor).unwrap();
		let act = String::from_utf8(cursor.into_inner()).unwrap();

		strict_assert_text(&act, EXPECTED_HTML.as_str())
	}

	#[test]
	fn extract() {
		let reg = regex::Regex::new(r"(?s)<svg.*?</svg>").unwrap();

		let binding = EXPECTED_FULL_SVG.as_str();
		let binding = reg.captures(binding).unwrap();
		let expected = binding.index(0);

		let act = extract_svg_element(EXPECTED_FULL_SVG.as_str()).unwrap();
		strict_assert_text(&act, expected)
	}

	#[test]
	fn step() {
		const FORMULA: &str = "{1+2*30}-{{42+4-5}*{{6+7}/2}}*{30+40*{20+4-1}}";
		let mut cursor = create_cursor();

		write_step_infix_html(FORMULA, &mut cursor).unwrap();

		let act = String::from_utf8(cursor.into_inner()).unwrap();
		let mut file = File::open("./test_artifacts/step_output.txt").unwrap();
		let mut expected = String::default();
		file.read_to_string(&mut expected).unwrap();

		strict_assert_text(&act, &expected);
	}
}
