//
// mod.rs
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/15/2021
// 
// Copywrite (c) 2021 Wess.io
//

mod token;

pub use token::Token;

use logos::Logos;
use std::convert::TryFrom;
use std::ops::Range as StdRange;
use text_size::{TextRange, TextSize};


pub struct Lexer<'a> {
  inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      inner: Token::lexer(input),
    }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = (&'a str, Token);

  fn next(&mut self) -> Option<Self::Item> {
    let kind = self.inner.next()?;
    let text = self.inner.slice();

    Some(
      (text, kind)
    )
  }
}
