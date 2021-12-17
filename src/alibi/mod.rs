//
// mod.rs
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/08/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::io::Read;

pub mod lexer;
pub mod ast;
pub mod parser;

use parser::Parser;
use ast::{
  Tree,
  Task,
};

pub struct Alibi {
  pub tree:Tree,
}

impl Alibi {
  pub fn new(filename:&str) -> Self {
    let source:String = file_read!(filename);
    let mut parser = Parser::new(&source);
    
    let tree = match parser.parse() {
      Some(t) => t,
      None => {
        panic!("Failed to parse alibi");
      }
    };

    Self { tree }
  }

  pub fn get_task(&self, name:&str) -> Option<Task>{
    let task = self.tree.tasks.iter().find(|t| t.name == name);

    match task {
      Some(t) => Some(t.clone()),
      None => None,
    }
  }
}
