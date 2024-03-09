use std::iter::Peekable;
use std::slice::Iter;

use anyhow::Error as AnyError;
use anyhow::Result as AnyResult;

#[derive(Debug)]
pub struct CommandOptions {
	rpn_expression: Option<String>,
	infix_expression: Option<String>,
	output_path: Option<String>,
}

impl CommandOptions {
	pub fn rpn_expression(&self) -> Option<String> {
		self.rpn_expression.clone()
	}

	pub fn infix_expression(&self) -> Option<String> {
		self.infix_expression.clone()
	}

	pub fn output_path(&self) -> Option<String> {
		self.output_path.clone()
	}
}

fn parse_rpn(input: &mut Peekable<Iter<String>>) -> AnyResult<String> {
	let mut ret = String::default();

	while let Some(elem) = input.peek() {
		if elem == &"-o" {
			break;
		} else {
			ret.push_str(elem);
			ret.push(' ');
		}

		input.next();
	}

	Ok(ret)
}

pub fn parse_command_options(input: Vec<String>) -> AnyResult<CommandOptions> {
	let mut opt = CommandOptions {
		rpn_expression: None,
		infix_expression: None,
		output_path: None,
	};

	let mut iter: Peekable<Iter<String>> = input.iter().peekable();
	iter.next();

	while let Some(elem) = iter.peek() {
		if elem == &"-r" {
			iter.next();
			opt.rpn_expression = Some(parse_rpn(&mut iter)?);
		} else if elem == &"-o" {
			iter.next();
			let cur = iter.next();
			println!("Hit:{:?}", &cur);
			if let Some(path) = cur {
				opt.output_path = Some(path.to_string())
			} else {
				return Err(AnyError::msg("Path is not specified."));
			}

			iter.next();
		}
	}

	Ok(opt)
}
