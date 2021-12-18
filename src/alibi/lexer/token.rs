//
// token.rs
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/13/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::fmt;
use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Logos, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
#[repr(u16)]
pub enum Token {
  #[regex(r"[\r?\n]")]
  Newline,

  #[regex(r"[ \t]+")]
  Indent,

  #[token("=")]
  Assign,

  #[token("!")]
  Bang,

  #[token("@")]
  Mute,

  #[token(":")]
  Colon,

  #[token("$")]
  DollarSign,

  #[token("{")]
  OpenCurly,

  #[token("}")]
  CloseCurly,
  
  #[token("export")]
  Export,

  #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
  Identifier,

  #[regex(r"[-|--]?[a-zA-Z0-9_\-]*")]
  Argument,

  #[regex(r"-?[0-9]+")]
  Int,

  #[regex(r#""(?:\\"|\\'|[^"])*""#)]
  #[regex(r#"'(?:\\"|\\'|[^'])*'"#)]
  LiteralString,

  #[regex(r"##[ \t]?[^\n]*")]
  Document,

  #[regex(r"#[^\n]*")]
  Comment,

  #[error]
  Error,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(match self {
      Self::Newline => "Newline",
      Self::Indent => "Ident",
      Self::Assign => "=",
      Self::Bang => "!",
      Self::Mute => "@",
      Self::Colon => ":",
      Self::DollarSign => "$",
      Self::Identifier => "identifier",
      Self::Argument => "argument",
      Self::Int => "int",
      Self::LiteralString => "string",
      Self::Comment => "comment",
      Self::Error => "invalid token",
      Self::OpenCurly => "{",
      Self::CloseCurly => "}",
      Self::Export => "export",
      Self::Document => "document",
    })
  }
}