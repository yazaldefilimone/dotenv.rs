use base64::{engine::general_purpose, Engine as _}; // base64 encode and decode
use std::error::Error;

pub fn base64_encode(input: &[u8]) -> String {
  let mut output_buf = String::new();
  general_purpose::STANDARD.encode_string(input, &mut output_buf);
  output_buf
}

pub fn base64_decode(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
  let output_buf = general_purpose::STANDARD.decode(input)?;
  Ok(output_buf)
}
