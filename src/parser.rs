use std::iter::Peekable;
use crate::error::{CompilerResult, UnexpectedEof, UnexpectedToken};
use crate::lexer::Lexer;
use crate::syntax::{Definition, Expr};
use crate::token::Token;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a, T> From<T> for Parser<'a> where T: Into<Lexer<'a>> {
    fn from(value: T) -> Self {
        Parser { lexer: value.into().peekable() }
    }
}

macro_rules! expect_token {
    ($lexer:expr, $pat:pat, $result:expr) => {
        core::hint::must_use(match $lexer.next() {
            Some(Ok((_, $pat))) => CompilerResult::Ok($result),
            Some(Ok((range, token))) => CompilerResult::Err(Box::new(UnexpectedToken(range, token))),
            Some(Err(err)) => CompilerResult::Err(err),
            None => CompilerResult::Err(Box::new(UnexpectedEof)),
        })
    };

    ($lexer:expr, $pat:pat) => {
        expect_token!($lexer, $pat, ())
    }
}

macro_rules! look_ahead {
    ($lexer:expr, $pat:pat) => {
        matches!($lexer.peek(), Some(Ok((_, $pat))))
    };
}

impl<'a> Parser<'a> {
    pub fn parse(mut self) -> CompilerResult<Vec<Definition>> {
        let mut result = Vec::new();
        while self.lexer.peek().is_some() {
            result.push(self.parse_definition()?);
        }
        Ok(result)
    }

    pub fn parse_definition(&mut self) -> CompilerResult<Definition> {
        let _ = self.lexer.next_if(|token| matches!(token, Ok((_, Token::NewDefinition))));

        let name = expect_token!(self.lexer, Token::Definition(name), name)?;
        expect_token!(self.lexer, Token::Assign)?;
        let expr = self.parse_expr()?;
        Ok(Definition { name, expr })
    }

    pub fn parse_expr(&mut self) -> CompilerResult<Expr> {
        let mut lhs = if look_ahead!(self.lexer, Token::LParen) {
            let name = expect_token!(self.lexer, Token::LParen)?;
            let expr = self.parse_expr()?;
            let name = expect_token!(self.lexer, Token::RParen)?;
            expr
        } else if look_ahead!(self.lexer, Token::Ident(_)) {
            let name = expect_token!(self.lexer, Token::Ident(name), name)?;
            Expr::Local(name)
        } else if look_ahead!(self.lexer, Token::Definition(_)) {
            let name = expect_token!(self.lexer, Token::Definition(name), name)?;
            Expr::Global(name)
        } else {
            expect_token!(self.lexer, Token::Lambda)?;
            let param = expect_token!(self.lexer, Token::Ident(name), name)?;
            expect_token!(self.lexer, Token::Dot)?;
            let body = self.parse_expr()?;
            Expr::Lambda(param, Box::new(body))
        };

        loop {
            match self.lexer.peek() {
                None | Some(Ok((_, Token::RParen | Token::NewDefinition))) => break Ok(lhs),
                _ => lhs = Expr::Apply(Box::new(lhs), Box::new(self.parse_expr()?)),
            }
        }
    }
}