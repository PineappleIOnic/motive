//
// entry.rs
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/16/2021
// 
// Copywrite (c) 2021 Wess.io
//

use crate::alibi::lexer::Token;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entry {
  pub token:Token,
  pub text:String,
}
