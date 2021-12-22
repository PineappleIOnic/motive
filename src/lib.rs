//
// lib.rs
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/01/2021
// 
// Copywrite (c) 2021 Wess.io
//

#![allow(dead_code)]
#![allow(unused_imports)]

use std::{
  path::Path,
  env,
  process::{
    exit,
    Command
  },
  io::{
    Write,
    Read,
    stdout,
    stderr,
  }
};

extern crate logos;

#[macro_use]
extern crate oxide;

use clap::{
  App,
  AppSettings,
};

// use oxide::console_error;

pub mod result;
pub mod commands;
pub mod alibi;
pub mod runner;

pub use result::Result;

use alibi::Alibi;
use runner::Runner;

use commands::{
  init::Init,
};

// pub async fn run() -> Result<()> {
//   println!("Motive testing....");

//   let ali = Alibi::new("alibi");
//   let runner = Runner::new(ali);

//   runner.exec("foo");

//   Ok(())
// }

fn get_alibi() -> Alibi {
  let cwd = env::current_dir().unwrap();
  let ali_path = cwd.join("alibi");
  let ali_file = ali_path.into_os_string().into_string().unwrap();

  let ali:Alibi = if Path::new(&ali_file).exists() {
    Alibi::new(&ali_file)
  } else {
    console_error!("No alibi config found. Run 'motive init' to create one.");
    exit(1);
  };

  return ali;
}

pub async fn run() -> Result<()> {
  let mut app = App::new("Motive")
    .version(env!("CARGO_PKG_VERSION"))
    .about("Project assistant and task runner.")
    .before_help("\n")
    .setting(AppSettings::AllowExternalSubcommands)
    .setting(AppSettings::ArgRequiredElseHelp)
    .subcommand(Init::app());

  let mut help = Vec::new();
  app.write_help(&mut help).unwrap();

  let matches = app.get_matches();
  match matches.subcommand_name() {
    Some("init") => Init::run(),
    Some("list") => {
      let ali:Alibi = get_alibi();

      println!();
      console_info!("Tasks available in Alibi");

      let mut char_space = 2;
      for task in ali.tree.tasks.iter() {
        if task.name.len() > char_space {
          char_space = task.name.len();
        }
      }

      ali.tree.tasks.iter().map(|task| {
        let mut title:Vec<String> = vec![task.name.clone()];
        let diff = char_space - task.name.len();
        
        for _ in 0..diff {
          title.push(" ".to_string());
        }

        string!(format!("- {} : {}", title.join(""), task.description.trim_start()).as_str())
      })
      .collect::<Vec<String>>()
      .iter()
      .for_each(|line| {
        console_info!("{}", line);
      });
    },
    Some(cmd) => {
      Runner::new(get_alibi()).exec(cmd);
    },
    None => {}
  }

  println!();

  Ok(())
}
