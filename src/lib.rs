//
// lib.rs
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/01/2021
// 
// Copywrite (c) 2021 Wess.io
//

#[macro_use]
extern crate oxide;

use std::str;

use clap::{
  App,
  AppSettings,
};

use oxide::console_error;

pub mod result;
pub mod config;
pub mod commands;

mod trigger;

pub use result::Result;

use commands::{
  init::Init,
};

use config::Config;
use trigger::Trigger;


pub async fn run() -> Result<()> {
  let config = Config::read().unwrap();
  config.load_envs();

  let mut app = App::new("Motive")
    .version("0.0.1")
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
    Some(cmd) => {
      if Trigger::run(cmd.clone().to_string()) == false {
        let msg = 
          str::from_utf8(&help)
          .unwrap()
          .to_string()
          .trim()
          .replace("Motive 0.0.1", "")
          .replace("Project assistant and task runner.", "")
          .strip_prefix("\n\n\n")
          .unwrap()
          .to_string();

        println!();
        console_error!("* Command \"{}\" not found.", cmd);
        println!("{}\n", msg);
      }
    },
    None => {}
  }


  Ok(())
}