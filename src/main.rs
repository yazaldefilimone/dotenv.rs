mod cli;
mod commands;
mod crypto;
mod env;
mod storage;
use commands::{run_with_command, scanner_workspace};

fn main() {
  let matches = cli::command_line();
  match matches.subcommand() {
    Some(("run", sub_matches)) => {
      let command: Vec<String> = sub_matches.get_many("COMMAND").unwrap().cloned().collect();
      run_with_command(&command);
    }
    _ => scanner_workspace(),
  }
}
