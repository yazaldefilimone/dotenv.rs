#![allow(dead_code, unused_variables)]
use ignore::WalkBuilder;
use std::{ffi::OsStr, path::PathBuf};

pub fn scanner(root: &PathBuf) -> Vec<PathBuf> {
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
