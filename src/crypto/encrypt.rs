use aead::AeadCore;
use aes_gcm::aead::rand_core::OsRng;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use std::io;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub const ITERATIONS: u32 = 100_000;

/// Derives a key from the password and salt using PBKDF2 with HMAC-SHA256.
pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, ITERATIONS, &mut key);
    key
}

/// Encrypts the input file and writes the nonce and encrypted data to the output file.
pub fn encrypt_file(input_file: &str, key: &[u8]) -> io::Result<()> {
    let cipher = Aes256Gcm::new_from_slice(key).expect("Invalid key length");
    let mut rng = OsRng;
    let nonce = Aes256Gcm::generate_nonce(&mut rng);

    let mut input_file_handle = File::open(input_file)?;
    let output_file = format!("{}.enc", input_file);
    let mut output_file_handle = File::create(&output_file)?;

    let mut plaintext = Vec::new();
    input_file_handle.read_to_end(&mut plaintext)?;

    let ciphertext = cipher
        .encrypt(&nonce.into(), plaintext.as_ref())
        .expect("encryption failure");

    output_file_handle.write_all(&nonce)?;
    output_file_handle.write_all(&ciphertext)?;

    Ok(())
}

/// Decrypts the input file and writes the decrypted data to the output file.
pub fn decrypt_file(input_file: &str, key: &[u8]) -> io::Result<()> {
    let cipher = Aes256Gcm::new_from_slice(key).expect("Invalid key length");

    let mut input_file_handle = File::open(input_file)?;
    let output_file = Path::new(input_file).with_extension("");
    let mut output_file_handle = File::create(output_file)?;

    let mut nonce = [0u8; 12]; // This should match the size of the nonce used during encryption
    input_file_handle.read_exact(&mut nonce)?;

    let mut ciphertext = Vec::new();
    input_file_handle.read_to_end(&mut ciphertext)?;

    let plaintext = cipher
        .decrypt(&nonce.into(), ciphertext.as_ref())
        .expect("decryption failure");

    output_file_handle.write_all(&plaintext)?;

    Ok(())
}
