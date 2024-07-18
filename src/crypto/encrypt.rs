use base64::{engine::general_purpose, Engine as _};
use hex;
use ring::aead;
use ring::rand::{SecureRandom, SystemRandom};
use secp256k1::PublicKey;
use std::error::Error;

use crate::utils::NUMBER_USED_ONE_BYTE_LENGTH;

pub fn encrypt_value(value: &str, public_key_str: &str) -> Result<String, Box<dyn Error>> {
  let public_key_bytes = hex::decode(public_key_str)?;
  let public_key = PublicKey::from_slice(&public_key_bytes)?;

  let rng = SystemRandom::new();
  let mut nonce = [0u8; NUMBER_USED_ONE_BYTE_LENGTH];
  rng.fill(&mut nonce).expect("Error: failed to fill nonce");

  let sealing_key = aead::UnboundKey::new(&aead::AES_256_GCM, public_key.serialize().as_ref()).unwrap();
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
  Ok(format!("encrypted:{}", encrypted_value))
}

pub fn base64_encode(input: &[u8]) -> String {
  let mut output_buf = String::new();
  general_purpose::STANDARD.encode_string(input, &mut output_buf);
  output_buf
}
