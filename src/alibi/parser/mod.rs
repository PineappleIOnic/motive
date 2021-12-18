//
// mod.rs
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/16/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::iter::Peekable;
use logos::Logos;

use oxide::{
  console_info,
};

use super::lexer::{
  Lexer, 
  Token, 
};

use super::ast::{
  EnvVar,
  Assignment,
  Task,
  TaskCommand,
  Tree,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entry {
  pub token:Token,
  pub text:String,
}

pub struct Parser<'a> {
  lexer: Peekable<Lexer<'a>>,
  env_vars: Vec<EnvVar>,
  assignments: Vec<Assignment>,
  tasks: Vec<Task>,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      lexer: Lexer::new(input).peekable(),
      env_vars: Vec::new(),
      assignments: Vec::new(),
      tasks: Vec::new(),
    }
  }

  pub fn bump(&mut self) {
    match self.lexer.next() {
      Some((_text, _kind)) => {},
      None => {},
    }
  }

  pub fn peek(&mut self) -> Option<Entry> {
    self.lexer.peek().map(|(text, kind)| Entry{text: string!(*text), token: *kind})
  }

  pub fn parse_assign(&mut self, identifier:Entry) {
    self.bump();
    
    let  peeked= self.peek();

    self.assignments.push(
      Assignment {
        identifier: identifier.text,
        value: match peeked {
          Some(entry) => entry.text.replace("\"", "").replace("'", "").clone(),
          None => "".to_string(),
        },
      }
    );
  }

  pub fn parse_task_command(&mut self) -> Option<TaskCommand> {
    let mut line:Vec<String> = Vec::new();
    let mut muted = false;

    while let Some(peeked) = self.peek() {
      match peeked.token {
        Token::Mute => {
          if line.is_empty() {
            muted = true;
          }
        },
        Token::Newline => {
          if line.is_empty() { return None; }

          return Some(TaskCommand {
            line,
            muted,
          });
        },
        Token::Identifier | Token::Argument | Token::LiteralString | Token::Int  => {
          line.push(peeked.text);
        },
        _ => {}
      }

      self.bump();
    }

    None
  }

  pub fn parse_task_subtasks(&mut self) -> Option<Vec<String>> {
    let mut subtasks:Vec<String> = Vec::new();

    self.bump();

    while let Some(peeked) = self.peek() {
      match peeked.token {
        Token::Newline => {
          return Some(subtasks);
        },
        Token::Identifier => {
          subtasks.push(peeked.text);
        },
        _ => {}
      }

      self.bump();
    }
    
    None
  }

  pub fn parse_task(&mut self, identifier:Entry, doc:String) -> Option<Task> {
    
    let subtasks = self.parse_task_subtasks().unwrap_or(Vec::new());
    let mut commands:Vec<TaskCommand> = Vec::new();

    while let Some(peeked) = self.peek() {
      match peeked.token {
        Token::Indent => {
          if let Some(cmd) = self.parse_task_command() {
            commands.push(cmd);
          }
        },
        Token::Newline => {
          self.bump();

          continue;
        },
        _ => {
          break;
        }
      }
      
      self.bump();
    }

    Some(
      Task {
        name: identifier.text,
        description: doc,
        subtasks: subtasks.clone(),
        commands,
      }
    )
  }

  pub fn parse(&mut self) -> Option<Tree> {
    let mut doc = string!("");

    while let Some(entry) = self.peek() {
      self.bump(); 

      match entry.token {
        Token::Export => {
          let mut env_var = string!("");
          let mut env_val = string!("");

          while let Some(peeked) = self.peek() {
            match peeked.token {
              Token::Newline => { break; },
              Token::Identifier | Token::LiteralString => {
                if env_var.is_empty() == false && env_val.is_empty() == false {
                  break;
                }

                if env_var.is_empty() {
                  env_var = peeked.text;
                } else {
                  env_val = peeked.text;
                }

                self.bump();
              },
              _ => {
                self.bump();
              },
            }
          }

          self.env_vars.push(
            EnvVar {
              key: env_var,
              value: env_val,
            }
          );

          continue;
        },

        Token::Identifier => {
          let following = self.peek();

          if following.is_none() {
            self.bump();

            break;
          }

          let following = following.unwrap();

          match following.token {
            Token::Assign => self.parse_assign(entry),
            Token::Colon => {
              match self.parse_task(entry, doc.clone()) {
                Some(task) => {
                  self.tasks.push(task);
                  doc = string!("");
                },
                None => { continue; },
              }
            },
            _ => { 
              self.bump(); 
            },
          }
        },
        Token::Document => {
          doc = entry.text.clone().replace("##", "");
        }
        _ => { 
          continue; 
        }
      }
    }

    Some(Tree {
      assignments: self.assignments.clone(),
      tasks: self.tasks.clone(),
      env_vars: self.env_vars.clone(),
    })
  }
}
