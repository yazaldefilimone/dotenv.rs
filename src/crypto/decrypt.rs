#![allow(dead_code, unused_variables)]

use crate::utils::NUMBER_USED_ONE_BYTE_LENGTH;

pub fn decrypt(encrypted_value: &str, private_key_str: &str) -> Result<String, Box<dyn Error>> {
  let private_key_bytes = hex::decode(private_key_str)?;
  let private_key = PrivateKey::from_slice(&private_key_bytes)?;

  let mut encrypted_value_bytes = base64::decode(encrypted_value)?;
  let nonce = encrypted_value_bytes[..NUMBER_USED_ONE_BYTE_LENGTH].to_vec();
  let encrypted_value_bytes = encrypted_value_bytes[NUMBER_USED_ONE_BYTE_LENGTH..].to_vec();

  let opening_key = aead::UnboundKey::new(&aead::AES_256_GCM, private_key.serialize().as_ref()).unwrap();
  let opening_key = aead::LessSafeKey::new(opening_key);

  let mut decrypted_value = Vec::new();
  let add_auth_data = aead::Aad::empty();

  opening_key
    .open_in_place(
      aead::Nonce::assume_unique_for_key(nonce),
      add_auth_data,
      &mut encrypted_value_bytes,
      &mut decrypted_value,
    )
    .unwrap();

  Ok(String::from_utf8(decrypted_value)?)
}
