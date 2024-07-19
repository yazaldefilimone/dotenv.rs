#![allow(dead_code, unused_variables)]

use hex;
use ring::aead;
use ring::digest;
use ring::rand::{SecureRandom, SystemRandom};
// use secp256k1::PublicKey;
use std::error::Error;

use super::base64::base64_encode;
use crate::utils::{ENCRYPTED_VALUE_PREFIX, NONCE_BYTYES_LENGTH};

pub fn encrypt_value(value: &str, public_key_str: &str) -> Result<String, Box<dyn Error>> {
  let public_key_bytes = hex::decode(public_key_str).expect("Error: invalid public key");
  // let public_key = PublicKey::from_slice(&public_key_bytes).expect("Error: invalid public key");

  let rng = SystemRandom::new();
  let mut nonce = [0u8; NONCE_BYTYES_LENGTH];
  rng.fill(&mut nonce).expect("Error: failed to fill nonce");

  // Derive a symmetric key from the public key using SHA-256
  let public_key_hash = digest::digest(&digest::SHA256, &public_key_bytes[1..]); // Remove o primeiro byte (0x04)

  let sealing_key = aead::UnboundKey::new(&aead::AES_256_GCM, public_key_hash.as_ref()).unwrap();
  let sealing_key = aead::LessSafeKey::new(sealing_key);

  let mut in_out = value.as_bytes().to_vec();

  let nonce_time_key = aead::Nonce::assume_unique_for_key(nonce);
  let add_auth_data = aead::Aad::empty();

  sealing_key
    .seal_in_place_append_tag(nonce_time_key, add_auth_data, &mut in_out)
    .unwrap();

  let mut encrypted_value = nonce.to_vec();
  encrypted_value.extend_from_slice(&in_out);

  let encrypted_value = base64_encode(&encrypted_value);
  let encrypted_value = format!("{}{}", ENCRYPTED_VALUE_PREFIX, encrypted_value);
  Ok(encrypted_value)
}
