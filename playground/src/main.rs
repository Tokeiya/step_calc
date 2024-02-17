use std::io::prelude::*;
use std::io::Write;

use combine::{ParseError, Parser, Stream};

use parser_sample::*;
mod parser_sample;
use combine as cmb;
use combine::parser::char as chr;

fn main() {
	let mut num_parser = cmb::many1(chr::digit::<&str>()).map(|x: String| x);

	let mut hoge = (chr::spaces(), num_parser, chr::spaces()).map(|(_, x, _)| x);

	let a = hoge.parse("200");
}

fn foo<Input>(parser: impl Parser<Input, Output = String>) -> impl Parser<Input, Output = String>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(chr::spaces(), parser, chr::spaces()).map(|(_, x, _)| x)
}
