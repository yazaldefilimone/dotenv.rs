#![allow(dead_code, unused_variables)]
use super::base64::base64_decode;
use crate::utils::{ENCRYPTED_VALUE_PREFIX, NONCE_BYTYES_LENGTH};
use hex;
use ring::aead;
use secp256k1::SecretKey;
use std::error::Error;

pub fn decrypt_value(value: &str, private_key_str: &str) -> Result<String, Box<dyn Error>> {
  if !value.starts_with(ENCRYPTED_VALUE_PREFIX) {
    return Err("Error: value is not encrypted".into());
  }

  let encrypted_value = value.trim_start_matches(ENCRYPTED_VALUE_PREFIX);

  if encrypted_value.is_empty() {
    return Err("Error: encrypted value is empty".into());
  }

  let encrypted_bytes = base64_decode(encrypted_value).expect("Error: invalid base64 value");

  let nonce = &encrypted_bytes[..NONCE_BYTYES_LENGTH];
  let mut ciphertext = encrypted_bytes[NONCE_BYTYES_LENGTH..].to_vec();

  let private_key_bytes = hex::decode(private_key_str).unwrap();
  let private_key = SecretKey::from_slice(&private_key_bytes).unwrap();

  let sealing_key = aead::UnboundKey::new(&aead::AES_256_GCM, private_key.as_ref());

  if sealing_key.is_err() {
    return Err("Error: failed to create sealing key".into());
  }

  let opening_key = aead::LessSafeKey::new(sealing_key.unwrap());

  let one_time_key = aead::Nonce::try_assume_unique_for_key(nonce).unwrap();

  let add_auth_data = aead::Aad::empty();

  let decrypted_data = opening_key.open_in_place(one_time_key, add_auth_data, &mut ciphertext);

  if decrypted_data.is_err() {
    return Err("Error: failed to decrypt value".into());
  }

  let decrypted_value = String::from_utf8(decrypted_data.unwrap().to_vec())?;

  Ok(decrypted_value)
}
