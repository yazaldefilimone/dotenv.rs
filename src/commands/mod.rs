#![allow(dead_code, unused_variables)]

use crate::{crypto, env};
use std::process::{Command, Stdio};

pub fn run(command: &Vec<String>) {
  let cmd = command.get(0).expect("Error: no command found");
  let args: Vec<&str> = command.iter().skip(1).map(|s| s.as_str()).collect();
  let envs = env::loader::load_workspace();
  for (key, value) in envs {
    std::env::set_var(key, value);
  }

  let mut child = Command::new(cmd)
    .args(&args)
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .spawn()
    .expect("Error: failed to execute command");
  child.wait().expect("Error: failed to wait for child");
}

pub fn set_env(key: &str, value: &str) {
  std::env::set_var(key, value);
}

// generate keys, private and public
pub fn generate_keys() -> crypto::keys::Keys {
  let keys = crypto::keys::create_new_keys();
  // todo: save private key and print public key
  return keys;
}
