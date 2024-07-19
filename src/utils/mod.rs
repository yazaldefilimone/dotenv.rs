#![allow(dead_code, unused_variables)]

use std::{fs::File, io::Write};

pub fn get_env_value(key: &str) -> Option<String> {
  match std::env::var(key) {
    Ok(val) => Some(val),
    Err(_) => None,
  }
}

pub fn get_env_value_or(key: &str, default: &str) -> String {
  match std::env::var(key) {
    Ok(val) => val,
    Err(_) => default.to_string(),
  }
}

pub fn set_env_value(key: &str, value: &str) {
  std::env::set_var(key, value);
}

pub fn delete_env_value(key: &str) {
  std::env::remove_var(key);
}

pub fn check_env_value_is_encrypted(value: &str) -> bool {
  return value.starts_with(ENCRYPTED_VALUE_PREFIX);
}

// env files
pub const ENV_FILE: &str = ".env";
pub const ENV_FILE_DOT: &str = ".env.";

// private keys env file
pub const PRIVATE_KEYS_ENV_FILE: &str = ".env.keys";

// public keys env file
pub const PUBLIC_KEYS_ENV_FILE: &str = ".env.keys.pub";

// default private key name
pub const DEFAULT_PUBLIC_KEY_NAME: &str = "DOTENV_PUBLIC_KEY_PRODUCTION";
pub const DEFAULT_PRIVATE_KEY_NAME: &str = "DOTENV_PRIVATE_KEY_PRODUCTION";

// number used one time byte length
pub const NONCE_BYTYES_LENGTH: usize = 12;

// encrypted value
pub const ENCRYPTED_VALUE_PREFIX: &str = "encrypted:";

pub fn generate_private_key(key: &str, file: &mut File) {
  let content = format!(
    "# private decryption keys. DO NOT commit to source control \n\n\n{}={}",
    DEFAULT_PRIVATE_KEY_NAME, key
  );
  file.write_all(content.as_bytes()).unwrap();
}

pub fn generate_public_key(key: &str, file: &mut File) {
  let content = format!(
    "# public encryption keys. Is ok to share with others \n\n\n{}={}",
    DEFAULT_PUBLIC_KEY_NAME, key
  );
  file.write_all(content.as_bytes()).unwrap();
}
