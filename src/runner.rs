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

use regex::Regex;

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

  fn var_check(&self, args:&Vec<String>) -> Vec<String> {
    let assignments = 
      self.alibi.tree.assignments
      .iter()
      .map(|a| a.identifier.clone())
      .collect::<Vec<String>>();

    let pattern = Regex::new(r"\$\{?([a-zA-Z0-9_]+)\}?").unwrap();
    let mut new_args:Vec<String> = Vec::new();

    for arg in args {
      let mut new_arg = arg.clone();
      
      for cap in pattern.captures_iter(arg) {
        let var = string!(cap.get(1).unwrap().as_str());
        if assignments.contains(&var) {
          let value = 
            self.alibi.tree.assignments
            .iter()
            .find(|a| a.identifier == var).unwrap()
            .value
            .clone()
            .replace("\"", "")
            .replace("'", "");
          
          new_arg = value.clone();
        }
      }

      new_args.push(new_arg);
    }
    
    new_args
  }

  fn run(&self, cmd:&TaskCommand) {
    let mut command = cmd.line.clone();
    let mut args = command.split_off(1);
    args = self.var_check(&args);

    let name = command.first().unwrap();

    let mut env_vars = HashMap::new();
    
    for env_var in &self.alibi.tree.env_vars {
      let key = env_var.key.clone();
      let value = env_var.value.clone();

      env_vars.insert(key, value);
    }


    let mut run_args:Vec<String> = vec![name.clone()];
    run_args.append(&mut args);

    let result = 
      Command::new(env!("SHELL"))
      .envs(&env_vars)
      .arg("-c")
      .arg(run_args.join(" "))
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
