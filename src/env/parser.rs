#![allow(dead_code, unused_variables)]
use rayon::prelude::*;
use std::io::Read;
use std::path::PathBuf;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parser(env_paths: &Vec<PathBuf>) -> Result<Vec<(String, String)>, String> {
  let mut parsed_lines: Vec<(String, String)> = Vec::new();
  for path in env_paths {
    let lines = read_file_and_return_lines(path).unwrap();
    parsed_lines.extend(parse_lines(&lines).unwrap());
  }
  Ok(parsed_lines)
}
// Read file and return non-empty lines
pub fn read_file_and_return_lines(path: &PathBuf) -> Result<Vec<String>, String> {
  let file = File::open(path).map_err(|e| e.to_string())?;
  let reader = BufReader::new(file);
  let lines: Vec<String> = reader
    .lines()
    .filter_map(|line| match line {
      Ok(line) => {
        if line.is_empty() {
          return None;
        }
        // ignore comments
        if line.starts_with("#") {
          return None;
        }
        return Some(line);
      }
      Err(_) => None,
    })
    .collect();
  Ok(lines)
}

// Parse a single line into a key-value pair
fn parse_line(line: &str) -> Result<(String, String), String> {
  let mut parts = line.splitn(2, '=');
  let key = parts.next().ok_or("No key found")?.trim();
  let value = parts.next().ok_or("No value found")?.trim();
  Ok((key.to_string(), value.to_string()))
}

fn parse_lines(lines: &Vec<String>) -> Result<Vec<(String, String)>, String> {
  let mut parsed_lines: Vec<(String, String)> = Vec::new();
  for line in lines {
    if line.starts_with("#") {
      continue;
    }
    if let Ok((key, value)) = parse_line(&line) {
      parsed_lines.push((key, value));
    }
  }
  return Ok(parsed_lines);
}
