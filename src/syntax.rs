#[derive(Debug, Clone)]
pub struct Definition {
    name: String,
    value: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Lambda(String, Box<Expr>),
    Local(String),
    Global(String),
}