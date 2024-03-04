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

    let args: Vec<String> = env::args().collect();

    println!("{:?}", &args);

    let opt = parse_command_options(args)?;

    println!("{:?}", &opt);

    if opt.output_path().is_none() {
        println!("Output path is not specified.");
        return Err(AnyError::msg("Output path is not specified."));
    }

    let file = File::create(opt.output_path().unwrap())?;

    let a = &opt.rpn_expression().unwrap().replace('"', "");

    rpn_html_writer::write_html(a, file)?;

    Ok(())
}
