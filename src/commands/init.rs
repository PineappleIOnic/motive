//
// init.rs
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/01/2021
// 
// Copywrite (c) 2021 Wess.io
//

use clap::{App};

use oxide::{
  console_error,
  console_panic,
  console_success,
};

use crate::{
  config::Config,
};

pub struct Init {}

impl Init {
  pub fn app() -> App<'static> {
    App::new("init")
    .about("Creates a new 'alibi' file in current directory")  
  }

  pub fn run() {
    if Config::exists() {
      console_error!("alibi file already exists");
      return;
    }

    match Config::write_default() {
      Ok(_) => console_success!("alibi file created"),
      Err(why) => console_panic!("failed to create alibi file: {}", why),
    };
  }
}
