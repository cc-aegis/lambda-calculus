#![feature(hint_must_use)]

use crate::parser::Parser;

mod token;
mod syntax;
mod lexer;
mod error;
mod parser;

fn main() {
    let src = "SUCC := λn.λf.λx.f (n f x)\n0 := λf.λx.x";
    let parser = Parser::from(src);
    let expr = parser.parse();
    println!("{expr:?}");
}
