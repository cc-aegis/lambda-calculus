use crate::lexer::Lexer;

mod token;
mod syntax;
mod lexer;
mod error;

fn main() {
    let src = "SUCC := λn.λf.λx.f (n f x)";
    let lexer = Lexer::from(src);
    let tokens = lexer.collect::<Result<Vec<_>, _>>();
    println!("{tokens:?}");
}
