mod commands;
mod crypto;
mod env;
mod storage;
mod utils;

pub fn run_in_command_line(command: Vec<String>) {
  commands::run(&command);
}

pub fn scanner_workspace() -> Vec<(String, String)> {
  return env::loader::load_workspace();
}
