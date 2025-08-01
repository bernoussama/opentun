use x25519_dalek::{PublicKey, StaticSecret};

pub type PrivateKeyBytes = [u8; 32];
pub type PublicKeyBytes = [u8; 32];

/// generate a new X25519 keypair ()->(private_key, public_key)
pub fn generate_keypair() -> (PrivateKeyBytes, PublicKeyBytes) {
    let private_key = StaticSecret::random();
    let public_key = PublicKey::from(&private_key);
    (private_key.to_bytes(), public_key.to_bytes())
}

pub fn gen_base64_private_key() -> String {
    let private_key = StaticSecret::random();
    base64::encode(private_key.to_bytes())
}

pub fn gen_base64_public_key(private_key: &str) -> crate::Result<String> {
    let private_key_bytes = base64::decode(private_key).expect("Invalid base64 private key");
    if private_key_bytes.len() != 32 {
        return Err(crate::IpouError::InvalidKeyLength(private_key_bytes.len()));
    }
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&private_key_bytes);
    let private_key = StaticSecret::from(key_array);
    let public_key = PublicKey::from(&private_key);
    Ok(base64::encode(public_key.to_bytes()))
}
