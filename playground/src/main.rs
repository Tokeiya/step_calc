use std::fmt::{Arguments, Debug, Display};

#[allow(dead_code)]
mod infix_html_writer;
mod rpn_html_writer;
mod test_helper;

mod console_color;
mod option_parser;
#[cfg(test)]
mod test_writer;

fn main() {
	foo(format_args!("hello {:?}","rust"))
	
}


fn foo(args:Arguments){
	let a=format_args!("{}{}",args,args);
	println!("{} {}",a,a);
}
