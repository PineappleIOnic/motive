//
// ast.rs
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/10/2021
// 
// Copywrite (c) 2021 Wess.io
//

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
  Integer,
  String,
  Script,
  None,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnvVar {
  pub key:String,
  pub value:String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Assignment {
  pub identifier: String,
  pub value:String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskCommand {
  pub line:Vec<String>,
  pub muted:bool,
}

impl TaskCommand {
  pub fn default() -> Self {
    Self {
      line: Vec::new(),
      muted: false,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Task {
  pub name: String,
  pub subtasks: Vec<String>,
  pub commands: Vec<TaskCommand>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tree {
  pub assignments: Vec<Assignment>,
  pub tasks: Vec<Task>,
  pub env_vars: Vec<EnvVar>,
}