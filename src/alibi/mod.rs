//
// mod.rs
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/08/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::iter::Peekable;
use logos::Logos;

mod lexer;
use lexer::{
  Lexer, 
  Token, 
};

mod ast;
use ast::{
  Assignment,
  Task,
  TaskCommand,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Entry {
  pub token:Token,
  pub text:String,
}

struct Parser<'a> {
  lexer: Peekable<Lexer<'a>>,
  assignments: Vec<Assignment>,
  tasks: Vec<Task>,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      lexer: Lexer::new(input).peekable(),
      assignments: Vec::new(),
      tasks: Vec::new(),
    }
  }

  pub fn bump(&mut self) {
    match self.lexer.next() {
      Some((_text, _kind)) => {},
      None => {
        println!("EOF");
      },
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

    let mut entered = false;

    while let Some(peeked) = self.peek() {
      match peeked.token {
        Token::Newline => {
          return Some(TaskCommand {
            line,
            muted,
          });
        },
        Token::Mute => {
          if entered == false {
            muted = true;
          }
        },
        Token::Identifier => {
          line.push(peeked.text);
        },
        Token::Argument => {
          line.push(peeked.text);
        },
        _ => {
          
        }
      }

      entered = true;
      self.bump();
    }

    None
  }

  pub fn parse_task(&mut self, identifier:Entry) {
    self.bump();

    while let Some(peeked) = self.peek() {
      let mut commands:Vec<TaskCommand> = Vec::new();

      match peeked.token {
        Token::Indent => {
          self.bump();
          
          if let Some(cmd) = self.parse_task_command() {
            commands.push(cmd);
          }
        },
        Token::Identifier => {
          self.tasks.push(
            Task {
              name: identifier.text.clone(),
              subtasks: vec![],
              commands,
            }
          );

          return;
        },
        _ => {
          println!("Peeeked: {:?}", peeked);
        }
      }

      self.bump();
    }
  }

  pub fn parse(&mut self) {
    while let Some(entry) = self.peek() {
      self.bump(); 

      match entry.token {
        Token::Identifier => {
          let following = self.peek();

          if following.is_none() {
            self.bump();

            break;
          }

          let following = following.unwrap();

          match following.token {
            Token::Assign => self.parse_assign(entry),
            Token::Colon => self.parse_task(entry),
            _ => { self.bump(); },
          }
        },
        _ => {}
      }
    }

    println!("Assignments: {:?}", self.assignments);
    println!("Tasks: {:?}", self.tasks);
  }
}

pub fn parse(source: &str) {
  // let parsed = Lexer::new(source).collect::<Vec<_>>();

  // println!("{:#?}", parsed);

  let mut parser = Parser::new(source);
  parser.parse();
}
