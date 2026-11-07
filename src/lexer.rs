use std::iter::{Enumerate, Peekable};
use std::ops::RangeInclusive;
use std::str::Chars;
use crate::error::{CompilerResult, UnexpectedChar, UnexpectedEof};
use crate::token::Token;

pub type Ranged<T> = (RangeInclusive<usize>, T);

pub struct Lexer<'a> {
    chars: Peekable<Enumerate<Chars<'a>>>,
}

impl Lexer<'_> {
    fn skip_whitespace(&mut self) {
        while self.chars.next_if(|&(_, c)| c.is_whitespace()).is_some() {}
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(value: &'a str) -> Self {
        Lexer { chars: value.chars().enumerate().peekable() }
    }
}

impl Iterator for Lexer<'_> {
    type Item = CompilerResult<Ranged<Token>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        match self.chars.next()? {
            (idx, 'λ') => Some(Ok((idx..=idx, Token::Lambda))),
            (idx, '.') => Some(Ok((idx..=idx, Token::Dot))),
            (idx, '(') => Some(Ok((idx..=idx, Token::LParen))),
            (idx, ')') => Some(Ok((idx..=idx, Token::RParen))),
            (start, ':') => match self.chars.next() {
                Some((end, '=')) => Some(Ok((start..=end, Token::Assign))),
                Some((idx, char)) => Some(Err(Box::new(UnexpectedChar(idx, char)))),
                None => Some(Err(Box::new(UnexpectedEof))),
            },
            (start, initial) if initial.is_ascii_alphanumeric() => {
                let mut name = initial.to_string();
                let mut end = start;
                while let Some((idx, char)) = self.chars.next_if(|&(_, c)| c.is_ascii_alphanumeric()) {
                    name.push(char);
                    end = idx;
                }
                if initial.is_lowercase() {
                    Some(Ok((start..=end, Token::Ident(name))))
                } else {
                    Some(Ok((start..=end, Token::Definition(name))))
                }
            },
            (idx, char) => Some(Err(Box::new(UnexpectedChar(idx, char)))),
        }
    }
}