//
// runner.rs
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/16/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::{
  collections::HashMap,
  process::{
    Command,
    Stdio
  },
  io::{
    Write,
    Read,
    stdout,
    stderr,
  }
};

use crate::result::Result;

use crate::alibi::Alibi;
use crate::alibi::ast::{
  Task, 
  TaskCommand
};


pub struct Runner {
  alibi:Alibi,
}

impl Runner {
  pub fn new(alibi:Alibi) -> Self {
    Self {
      alibi,
    }
  }

  fn run(&self, cmd:&TaskCommand) {
    let mut command = cmd.line.clone();
    let args = command.split_off(1);
    let name = command.first().unwrap();

    let mut env_vars = HashMap::new();
    
    for env_var in &self.alibi.tree.env_vars {
      let key = env_var.key.clone();
      let value = env_var.value.clone();

      env_vars.insert(key, value);
    }

    let result = 
      Command::new(name.clone())
      .envs(&env_vars)
      .args(
        args
        .iter()
        .map(|a| a.replace("\"", "").clone())
        .collect::<Vec<String>>()
      )
      .output()
      .expect(format!("Failed to run {}", name).as_str());

    if false == cmd.muted {
      stdout().write_all(&result.stdout).unwrap();
      stderr().write_all(&result.stderr).unwrap();
    }
  }

  pub fn exec(&self, cmd:&str) -> bool {
    let task = match self.alibi.get_task(cmd) {
      Some(t) => t,
      None => {
        return false;
      }
    };

    for c in task.commands.iter() {
      self.run(c);
    }

    for subtask in task.subtasks {
      if self.exec(&subtask) == false {
        return false;
      }
    }

    true
  }
}

// fn run_task_command(cmd:&TaskCommand) {
//   let mut command = cmd.line.clone();
//   let args = command.split_off(1);

//   execute(command.first().unwrap().clone(), args, cmd.muted);
// }
