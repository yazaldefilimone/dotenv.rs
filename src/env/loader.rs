#![allow(dead_code, unused_variables)]
use ignore::WalkBuilder;
use std::{
  ffi::OsStr,
  fs::File,
  io::{BufReader, Read},
  path::PathBuf,
};

use crate::utils;

use super::parser;

pub fn loader(root: &PathBuf) -> Vec<PathBuf> {
  let walkdir = WalkBuilder::new(root)
    .hidden(false)
    .ignore(true) // use .gitignore
    .git_ignore(true) // use .gitignore
    .git_global(true) // use .gitignore_global
    .git_exclude(true) // use .gitignore_exclude
    .build();

  let vector_envs: Vec<PathBuf> = walkdir.filter_map(mapper_workspace).collect();
  vector_envs
}

fn mapper_workspace(result_walker: Result<ignore::DirEntry, ignore::Error>) -> Option<PathBuf> {
  if let Ok(entry) = result_walker {
    let path = entry.path();
    if !path.is_file() {
      return None;
    }

    let file_name = path.file_name().unwrap().to_str().unwrap();

    // suport .env and .env.***
    if file_name.starts_with(".env") {
      return Some(path.to_path_buf());
    }
  }
  None
}

pub fn load_workspace() -> Vec<(String, String)> {
  let workspace = std::env::current_dir().unwrap();
  let workspace_envs = loader(&workspace);
  let parsed = parser(&workspace_envs);
  return parsed.unwrap();
}

pub fn load_public_keys() -> String {
  let current_dir = std::env::current_dir().unwrap();
  let public_keys = current_dir.join(utils::PUBLIC_KEYS_ENV_FILE);
  let mut public_keys = File::open(public_keys).unwrap();
  let mut content = String::new();
  public_keys.read_to_string(&mut content).unwrap();
  let lines: Vec<&str> = content.lines().collect();
  let public_value = find_key_in_each_line(lines, utils::DEFAULT_PUBLIC_KEY_NAME).expect("Error: public key not found");
  return public_value;
}

fn find_key_in_each_line(lines: Vec<&str>, key: &str) -> Option<String> {
  for line in lines {
    if line.starts_with(key) {
      let parts: Vec<&str> = line.splitn(2, '=').collect();
      return Some(parts[1].to_string());
    }
  }
  return None;
}

pub fn load_private_keys() -> String {
  let current_dir = std::env::current_dir().unwrap();
  let private_keys = current_dir.join(utils::PRIVATE_KEYS_ENV_FILE);
  let mut private_keys = File::open(private_keys).unwrap();
  let mut content = String::new();
  private_keys.read_to_string(&mut content).unwrap();
  let lines: Vec<&str> = content.lines().collect();
  let private_value =
    find_key_in_each_line(lines, utils::DEFAULT_PRIVATE_KEY_NAME).expect("Error: private key not found");
  return private_value;
}
