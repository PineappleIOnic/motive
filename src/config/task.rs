//
// task.rs
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/01/2021
// 
// Copywrite (c) 2021 Wess.io
//

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCommand {
  pub command: String,

  #[serde(skip_serializing)]
  #[serde(default)]
  pub args:Vec<String>,

  #[serde(skip_serializing)]
  #[serde(default)]
  pub silent:bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
  pub name:String,
  pub commands:Vec<TaskCommand>,
}
