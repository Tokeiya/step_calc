use std::fs::File;
use std::io::Cursor;
use std::io::{BufRead, BufReader};

mod html_writer;

fn main() {
	const FORMULA: &str = "{1+2*3}/{{4-5}*{{6+7}/2}}";

	let file = File::open("./playground/test_artifacts/full_html.txt").unwrap();
	let expected: Vec<_> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();

	let mut cursor = Cursor::<Vec<u8>>::default();
	html_writer::write_infix_html(FORMULA, &mut cursor).unwrap();

	cursor.set_position(0);
	let act: Vec<_> = cursor.lines().map(|x| x.unwrap()).collect();

	assert_eq!(act.len(), expected.len());

	for (exp, act) in expected.iter().zip(act.iter()) {
		assert_eq!(exp, act)
	}

	//let actual:Vec<String> =
}

// fn write_samples() {
// 	let current_dir = env::current_dir().unwrap();
// 	println!("The current directory is {}", current_dir.display());
// 	let tree = parse("{1+2*3}/{{4-5}*{{6+7}/2}}").unwrap().0.simplify();
// 	{
// 		let mut file = File::create("./playground/test_artifacts/sample.dot").unwrap();
// 		write_dot(&mut file, &tree).unwrap();
// 	}
//
// 	{
// 		let mut proc = Command::new("dot")
// 			.args(&[
// 				r"-Tsvg",
// 				r".\playground\test_artifacts\sample.dot",
// 				r"-o",
// 				r".\playground\test_artifacts\sample.svg",
// 			])
// 			.output()
// 			.unwrap();
//
// 		println!("{:?}", proc)
// 	}
// }
