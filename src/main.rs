mod cli;
mod commands;
mod crypto;
mod env;
mod storage;
mod utils;

use env::loader::load_workspace;

fn main() {
  println!("Hello, world, env: {:?}", std::env::var("API_URL"));
  let matches = cli::command_line();
  match matches.subcommand() {
    Some(("run", sub_matches)) => {
      let command: Vec<String> = sub_matches.get_many("COMMAND").unwrap().cloned().collect();
      commands::run(&command);
    }
    _ => {
      load_workspace();
    }
  }
}
