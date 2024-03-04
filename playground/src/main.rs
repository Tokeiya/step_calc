use std::env;
use std::fs::File;

use anyhow::{Error as AnyError, Result as AnyResult};

use option_parser::*;

#[allow(dead_code)]
mod infix_html_writer;
mod rpn_html_writer;
mod test_helper;

mod option_parser;
#[cfg(test)]
mod test_writer;

fn main() -> AnyResult<()> {
    rpn_html_writer::procedure()
}
