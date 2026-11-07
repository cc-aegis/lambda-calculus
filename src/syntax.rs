#[derive(Debug, Clone)]
pub struct Definition {
    pub name: String,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Lambda(String, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
    Local(String),
    Global(String),
}