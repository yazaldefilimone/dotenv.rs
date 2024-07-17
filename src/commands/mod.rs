#![allow(dead_code, unused_variables)]

use crate::env;
use std::process::{Command, Stdio};

pub fn run_with_command(command: &Vec<String>) {
  let cmd = command.get(0).expect("Error: no command found");
  let args: Vec<&str> = command.iter().skip(1).map(|s| s.as_str()).collect();

  scanner_workspace();

  let mut child = Command::new(cmd)
    .args(&args)
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .spawn()
    .expect("Error: failed to execute command");
  child.wait().expect("Error: failed to wait for child");
}

pub fn scanner_workspace() {
  let workspace = std::env::current_dir().unwrap();
  let workspace_envs = env::scanner(&workspace);
  let parsed = env::parser(&workspace_envs);
  if let Err(e) = &parsed {
    eprintln!("Error: {}", e);
  }
  println!("{} .env's, {} parsed", workspace_envs.len(), parsed.unwrap().len());
}
