#![allow(dead_code, unused_variables)]
use hex;
use ring::aead;
use secp256k1::PublicKey;
use std::error::Error;

use crate::utils::NUMBER_USED_ONE_BYTE_LENGTH;

use super::base64::base64_decode;

pub fn decrypt(encrypted_value: &str, private_key_str: &str) -> Result<String, Box<dyn Error>> {
  let private_key_bytes = hex::decode(private_key_str)?;
  let private_key = PrivateKey::from_slice(&private_key_bytes).expect("Error: failed to create private key");

  let mut encrypted_value_bytes = base64_decode(encrypted_value).expect("Error: failed to decode base64");
  let nonce = encrypted_value_bytes[..NUMBER_USED_ONE_BYTE_LENGTH].to_vec();
  let encrypted_value_bytes = encrypted_value_bytes[NUMBER_USED_ONE_BYTE_LENGTH..].to_vec();

  let opening_key = aead::UnboundKey::new(&aead::AES_256_GCM, private_key.serialize().as_ref()).unwrap();
  let opening_key = aead::LessSafeKey::new(opening_key);

  let mut decrypted_value = Vec::new();
  let add_auth_data = aead::Aad::empty();

  let one_time_key = aead::Nonce::assume_unique_for_key(nonce);

  opening_key
    .open_in_place(one_time_key, add_auth_data, &mut encrypted_value_bytes)
    .unwrap();

  Ok(String::from_utf8(decrypted_value)?)
}
