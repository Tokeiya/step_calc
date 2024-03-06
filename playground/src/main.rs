use std::fmt::{Arguments, Debug, Display};

#[allow(dead_code)]
mod infix_html_writer;
mod rpn_html_writer;
mod test_helper;

#[cfg(test)]
mod test_writer;

fn main() {
	foo(format_args!("{}", "hoge"));
}

fn foo(arg: Arguments<'_>) {
	println!("{}:{}", "env", arg)
}
