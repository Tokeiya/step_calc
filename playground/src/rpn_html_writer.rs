use std::collections::VecDeque;
use std::io::Write;

use anyhow::Result as AnyResult;

use parser::rpn::parser::Token;
use syntax::binary_operation::Operation;
use syntax::number_value::NumberValue;

fn token_to_string(scr: &VecDeque<Token>) -> String {
	let mut buff = String::default();

	for elem in scr.iter().rev() {
		match elem {
			Token::Number(num) => match num {
				NumberValue::Integer(i) => buff.push_str(&i.to_string()),
			},
			Token::Operator(op) => match op {
				Operation::Add => buff.push('+'),
				Operation::Sub => buff.push('-'),
				Operation::Mul => buff.push('*'),
				Operation::Div => buff.push('/'),
			},
		}

		buff.push(' ');
	}
	buff.remove(buff.len() - 1);
	buff
}

fn write_header(input: &VecDeque<Token>, writer: &mut dyn Write) -> AnyResult<()> {
	todo!()
}

fn write_state(input: &VecDeque<Token>, writer: &mut dyn Write) -> AnyResult<()> {
	todo!()
}

fn write_footer(writer: &mut dyn Write) -> AnyResult<()> {
	todo!()
}

pub fn write_html<T: Write>(input: &str, mut writer: T) -> AnyResult<()> {
	todo!()
}

#[cfg(test)]
mod tests {
	use parser::rpn as Rpn;
	use std::io::Cursor;

	use super::*;

	fn create_cursor() -> Cursor<Vec<u8>> {
		Cursor::<Vec<u8>>::default()
	}
	fn gen_token_stream() -> VecDeque<Token> {
		let (ret, rem) = Rpn::parser::tokenize("4 2 3 4 5 / + * -");
		assert!(rem.is_empty());
		ret
	}

	#[test]
	fn token_to_string_test() {
		let tokens = gen_token_stream();
		let act = token_to_string(&tokens);

		assert_eq!(&act, "4 2 3 4 5 / + * -");
	}

	#[test]
	fn write_header_test() {
		const EXPECTED: &str = r#"<!DOCTYPE html>
<html lang="ja">

<head>
    <meta charset="UTF-8">
    <style>
        table, th, td {
            border: 1px solid black;
            border-collapse: collapse;
        }

        th, td {
            padding: 8px;
            text-align: left;
        }
    </style>
    <title>4 2 3 4 5 / + * -</title>
</head>
<body>
<table>
"#;
		let mut cursor = create_cursor();
		write_header(&gen_token_stream(), &mut cursor).unwrap();

		todo!()
	}
}
