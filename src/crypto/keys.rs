#![allow(dead_code, unused_variables)]

use secp256k1::Secp256k1;

pub struct Keys {
  pub private_key: String,
  pub public_key: String,
}

impl Keys {
  pub fn new(private_key: String, public_key: String) -> Self {
    Self { private_key, public_key }
  }
}

pub fn create_new_keys() -> Keys {
  let secp = Secp256k1::new();
  let mut rng = rand::thread_rng();
  let (secret_key, public_key) = secp.generate_keypair(&mut rng);

  // let displayed_secret_key = secret_key.display_secret();
  // let private_key = displayed_secret_key.to_string();
  // let public_key = public_key.to_string();
  let private_key = hex::encode(secret_key.secret_bytes());
  let public_key = hex::encode(public_key.serialize_uncompressed());
  return Keys::new(private_key, public_key);
}
