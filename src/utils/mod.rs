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
  return value.starts_with("ENC:");
}

// env files
pub const ENV_FILE: &str = ".env";
pub const ENV_FILE_DOT: &str = ".env.";

// private keys env file
pub const PRIVATE_KEYS_ENV_FILE: &str = ".env.keys";

// public keys env file
pub const PUBLIC_KEYS_ENV_FILE: &str = ".env.keys.pub";

// default private key name
pub const DEFAULT_PUBLIC_KEY_NAME: &str = "XENV_PUBLIC_KEY";
pub const DEFAULTPRIVATE_KEY_NAME: &str = "XENV_PRIVATE_KEY";

// number used one time byte length
pub const NUMBER_USED_ONE_BYTE_LENGTH: usize = 12;
