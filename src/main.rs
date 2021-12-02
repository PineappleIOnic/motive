//
// main.rs
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/01/2021
// 
// Copywrite (c) 2021 Wess.io
//

use motive;

#[async_std::main]
async fn main() -> motive::Result<()> {
  motive::run().await
}
