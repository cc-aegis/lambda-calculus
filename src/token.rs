#[derive(Debug, Clone)]
pub enum Token {
    Definition(String),
    Assign,
    Lambda,
    Ident(String),
    Dot,
    LParen,
    RParen,
    NewDefinition,
}