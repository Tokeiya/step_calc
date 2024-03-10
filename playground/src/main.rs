use once_cell::sync::Lazy;
use regex::Regex;

fn remove_color_definition(scr: &str) -> String {
	static reg: Lazy<Regex> = Lazy::new(|| Regex::new(r"\x1B\[.+?m").unwrap());
	let array: Vec<_> = reg.find_iter(scr).collect();
	let mut ret = String::default();

	let mut s = 0usize;

	if array.is_empty() {
		scr.to_string()
	} else {
		for elem in array.iter() {
			ret.push_str(&scr[s..elem.start()]);
			s = elem.end();
		}

		ret
	}
}

fn main() {
	const txt: &str = "hello\t world";
	let a = remove_color_definition(txt);

	println!("{}", &a);
}
