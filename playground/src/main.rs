use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
	println!("{}", env::current_dir().unwrap().display())
}
