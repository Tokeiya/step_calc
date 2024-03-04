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

fn bar<const N:usize>()->[i32;N]{
    let mut a=[0;N];
    
    for i in 0..N {
        a[i]=(i+10) as i32;
    }
    
    for elem in a.iter_mut() {
        *elem+=100;
    }
    
    a

}

fn main() {
    let mut v=Vec::<String>::new();
    v.push("hello".to_string());
    v.push("world".to_string());
    foo(v.iter().map(|x|x.as_str()));
}

fn foo<'a>(iter:impl Iterator<Item=&'a str>){
    for elem in iter {
        println!("{elem}")
    }
}
