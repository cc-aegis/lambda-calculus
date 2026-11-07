use std::fmt::Debug;
use std::ops::RangeInclusive;
use crate::token::Token;

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

#[derive(Debug, Clone)]
pub struct UnexpectedToken(pub RangeInclusive<usize>, pub Token);

impl CompilerError for UnexpectedToken {

}