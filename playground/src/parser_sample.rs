use combine::{chainl1, choice, many1, ParseError, parser, Parser, Stream, token};
use combine::parser::char::{char, digit};

fn expr_<Input>() -> impl Parser<Input, Output=String>
	where Input: Stream<Token=char>,
	      Input::Error: ParseError<Input::Token, Input::Range, Input::Position>, {
	let unsigned = many1(digit()).map(|v: String| v);
	let signed = char('-').with(many1(digit()).map(|v: String| v)).map(|v: String| format!("-{}", v));
	
	let digit = choice((signed, unsigned));
	
	let bracket_expr = (char::<Input>('{'), expr(), char('}')).map(|(_, e, _)| format!("{{{}}}", e));
	
	let primary = choice((digit, bracket_expr));
	
	let op = choice((token::<Input>('*'), token('/')));
	let multitive_chain = op.map(|o: char| move |l: String, r: String| format!("({}{}{})", l, o, r));
	
	let op = choice((token::<Input>('+'), token('-')));
	let additive_chain = op.map(|o: char| move |l: String, r: String| format!("({}{}{})", l, o, r));
	
	let multitive = chainl1(primary, multitive_chain);
	
	let additive = chainl1(multitive, additive_chain);
	
	additive
}

parser! {
	pub fn expr[Input]()(Input)->String
	where [Input:Stream<Token = char>]{
		expr_()
	}
}

// fn main() {
// 	let ans = expr().parse("10+-2*3/{4+5}");
// 	println!("{:?}", ans);
// }
