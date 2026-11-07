use std::fmt::Debug;

pub type CompilerResult<T> = Result<T, Box<dyn CompilerError>>;

pub trait CompilerError: Debug {
    
}

#[derive(Debug, Clone)]
pub struct UnexpectedChar(pub usize, pub char);

impl CompilerError for UnexpectedChar {

}

#[derive(Debug, Clone)]
pub struct UnexpectedEof;

impl CompilerError for UnexpectedEof {

}