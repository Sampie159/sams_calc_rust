use crate::parser::parse;

mod calculator;
mod parser;
mod token;

fn main() {
    println!("Hello World!");
    parse("2 + 12() + 3");
}
