use std::fs::File;

mod html_writer;

fn main() {
	const FORMULA: &str = "{1+2*30}-{{42+4-5}*{{6+7}/2}}*{30+40*{20+4-1}}";
	let mut file = File::create("../test_artifacts/output.txt").unwrap();

	_ = regex::Regex::new("");

	html_writer::write_step_infix_html(FORMULA, &mut file).unwrap();
}
