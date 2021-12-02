//
// trigger.rs
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/01/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::{
  process::Command,
  io::{
    Write,
    stdout,
    stderr,
  }
};

use oxide::console_panic;

use crate::{
  config::{
    Config,
    task::Task,
  },
};

#[derive(Debug, Clone)]
pub struct Trigger {}

impl Trigger {
  pub fn run(task:String) -> bool {

    let config = Config::read().unwrap();

    let task_names = 
      config.tasks
      .clone()
      .unwrap()
      .iter()
      .map(|t| t.name.clone())
      .collect::<Vec<String>>();

    let callee:Task = match config.tasks.unwrap().into_iter().find(|t| t.name == task) {
      Some(t) => t,
      None => console_panic!("Task '{}' was not found in config", task)
    };

    callee.commands
    .clone()
    .into_iter()
    .for_each(|cmd| {
      if task_names.contains(&cmd.command) {
        Self::run(cmd.command.clone());
      } else {
        let command = cmd.command.clone();
        let args = cmd.args.clone();
        let silent = cmd.silent;

        let result = 
          Command::new(command.clone())
          .args(args)
          .output()
          .expect(format!("Failed to run {}", command).as_str());

        if false == silent {
          stdout().write_all(&result.stdout).unwrap();
          stderr().write_all(&result.stderr).unwrap();
        }
      }    
    });

    return true;
 }
}
