//
// mod.ts
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/01/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::{
  env,
  path::Path,
  fs::File,
  io::{
    self,
    Write,
    prelude::*,
  },
  collections::HashMap,
};


use serde_yaml::Value;

use serde::{
  Serialize,
  Deserialize,
};

pub mod task;

use task::{
  Task,
  TaskCommand,
};

const SILENT_MARKER:&str = "[_##SILENT##_]";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  pub tasks:Option<Vec<Task>>,
  pub env:Option<HashMap<String, Value>>,
}

impl Config {
  pub fn default() -> Self {
    Config {
      tasks: None,
      env: None,
    }
  }

  pub fn exists() -> bool {
    let cwd = env::current_dir().unwrap();
    let path = cwd.join("alibi");

    path.exists()
  }

  pub fn write(&self) -> Result<(), io::Error> {
    let cwd = env::current_dir().unwrap();
    let config = serde_yaml::to_string(self).unwrap();
    let file_path = format!("{}/alibi", cwd.to_str().unwrap());
    let path = Path::new(file_path.as_str());

    let mut file = match File::create(&path) {
      Ok(file) => file,
      Err(e) => {
        console_panic!("Unable to create alibi config file: {}", e);
      }
    };

    file.write_all(config.as_bytes())
  }

  pub fn read() -> Result<Self, io::Error> {
    let cwd = env::current_dir().unwrap();
    let path = cwd.join("alibi");

    let mut file = match File::open(&path) {
      Ok(file) => file,
      Err(e) => {
        console_panic!("Unable to open alibi config file: {}", e);
      }
    };

    let mut buffer = String::new();

    match file.read_to_string(&mut buffer) {
      Ok(_) => {},
      Err(e) => {
        console_panic!("Unable to read alibi config file: {}", e);
      }
    };

    buffer = buffer.replace("@", SILENT_MARKER);

    let data:HashMap<String, Value> = serde_yaml::from_str(&buffer).unwrap();
    let tasks = Self::get_tasks(&data);
    let env = Self::get_env(&data);

    Ok(Config {
      tasks: tasks,
      env: env,
    })
  }

  fn build_task_command(v:&Value) -> TaskCommand {
    let mut args:Vec<String> =
    v
    .as_str()
    .unwrap()
    .to_string()
    .split(" ")
    .map(|a| a.to_string())
    .collect();

    let mut command = args.remove(0);
    let mut silent = false;

    if command.starts_with("@") {
      silent = true;
      command = command.replace("@", "");
    }

    return TaskCommand {
      command,
      args,
      silent
    }
  }

  fn get_tasks(data:&HashMap<String, Value>) -> Option<Vec<Task>> {
    const RESERVED_KEYS:[&str; 3] = [
      "name",
      "description",
      "env",
    ];

    let tasks:Vec<Task> = data
    .into_iter()
    .filter(|(key, _)| !RESERVED_KEYS.contains(&key.as_str()))
    .map(|(k,v)| { 
      let name = k.to_string();
      
      let mut commands:Vec<TaskCommand> = Vec::new();

      if v.is_sequence() {
        commands.extend(
          v
          .as_sequence()
          .unwrap()
          .iter()
          .map(|v| {
            Self::build_task_command(v)
          })
        );
      } else {
        commands = vec![Self::build_task_command(&v)];
      }

      Task {
        name,
        commands,
      }
    })
    .collect();

    Some(tasks)
  }

  fn get_env(data:&HashMap<String, Value>) -> Option<HashMap<String, Value>> {
    if data.contains_key("env") == false {
      return None;
    }

    let env_vars = data.get("env").unwrap().as_mapping().unwrap();

    Some(
      env_vars
      .into_iter()
      .map(|(k, v)| {
        (k.as_str().unwrap().to_string(), v.clone())
      })
      .collect()
    )
  }

  pub fn load_envs(&self) {
    match self.env.clone() {
      Some(vars) =>
        vars
        .iter()
        .for_each(|(k,v)|  env::set_var(k, v.as_str().unwrap())),
      None => return
    };
  }
}