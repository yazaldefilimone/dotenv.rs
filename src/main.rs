#![allow(dead_code, unused_variables)]

mod cli;
mod commands;
mod crypto;
mod env;
mod storage;
mod utils;

use std::fs::File;

use env::loader::{self, load_workspace};
use utils::{generate_private_key, generate_public_key};

fn main() {
  let matches = cli::command_line();
  match matches.subcommand() {
    Some(("run", sub_matches)) => {
      let command: Vec<String> = sub_matches.get_many("COMMAND").unwrap().cloned().collect();
      commands::run(&command);
    }
    Some(("set", sub_matches)) => {
      let variables = sub_matches
        .get_many::<String>("VARIABLE")
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
      let file = sub_matches.get_one::<String>("file").unwrap();
      run_set(&variables, file)
    }
    Some(("generate-keys", _sub_matches)) => run_generate_keys(),
    Some(("encrypt", sub_matches)) => {
      let file = sub_matches.get_one::<String>("file");
      let public_key = sub_matches.get_one::<String>("public-key");
      run_encrypt(file, public_key);
    }
    _ => {
      load_workspace();
    }
  }
}

fn separate_variables(variables: &Vec<&String>) -> Vec<(String, String)> {
  if variables.len() % 2 != 0 {
    panic!("Error: variables must be even");
  }
  let mut result: Vec<(String, String)> = Vec::new();
  for i in (0..variables.len()).step_by(2) {
    let key = variables[i].clone();
    let value = variables[i + 1].clone();
    result.push((key, value));
  }
  return result;
}
fn run_set(variables: &Vec<&String>, file: &str) {
  let public_key = loader::load_public_keys();
  let key_values = separate_variables(variables);
  for (key, value) in key_values {
    let encrypted_value = crypto::encrypt::encrypt_value(value.as_str(), public_key.as_str());
    if encrypted_value.is_err() {
      panic!("Error: failed to encrypt value",);
    }
    println!("set {:?} with encryption ({})....", key, file);
    // println!("original value: {:?} and encrypted value: {:?}", value, encrypted_value);
  }
}

fn run_generate_keys() {
  let current_dir = std::env::current_dir().unwrap();
  let keys = crypto::keys::create_new_keys();
  println!("Public key: {:?}", keys.public_key);
  let mut env_keys_file = File::create(current_dir.join(utils::PRIVATE_KEYS_ENV_FILE)).unwrap();
  generate_private_key(keys.private_key.as_str(), &mut env_keys_file);
  let mut env_keys_pub_file = File::create(current_dir.join(utils::PUBLIC_KEYS_ENV_FILE)).unwrap();
  generate_public_key(keys.public_key.as_str(), &mut env_keys_pub_file);
}

fn run_in_command_line(command: Vec<String>) {
  commands::run(&command);
}

fn run_encrypt(file: Option<&String>, public_key: Option<&String>) {
  todo!("hei, why you not implement me :(");
}
