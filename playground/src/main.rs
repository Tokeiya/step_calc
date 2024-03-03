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
    // let file=File::create("rpn_first_light.html").unwrap();
    //
    // write_html("16 8 4 2 - * +",&file).unwrap()

    let args: Vec<String> = env::args().collect();
    // let mut args = Vec::<String>::default();
    //
    // args.push("F:\\step_calc\\target\\debug\\playground.exe".to_string());
    // args.push("-r".to_string());
    // args.push("10 20 +".to_string());
    // args.push("-o".to_string());
    // args.push("output.html".to_string());

    println!("{:?}", &args);

    let opt = parse_command_options(args)?;

    println!("{:?}", &opt);

    if opt.output_path().is_none() {
        println!("Output path is not specified.");
        return Err(AnyError::msg("Output path is not specified."));
    }

    let file = File::create(opt.output_path().unwrap())?;

    let a = &opt.rpn_expression().unwrap().replace('"', "");
    //println!("input:{a}");
    rpn_html_writer::write_html(a, file)?;

    Ok(())
}
