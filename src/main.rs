extern crate aes;
extern crate block_modes;
extern crate sha2;

use aes::Aes256;
use block_modes::{BlockMode, Cbc, block_padding::Pkcs7};
use sha2::{Digest, Sha256};
use std::env;
use openssl::rand::rand_bytes;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: <encrypt|decrypt> <text> <password>");
        return;
    }

    let action = &args[1];
    let text = &args[2];
    let password = &args[3];

    // Hash the password to get a 32-byte key
    let key = Sha256::digest(password.as_bytes());

    match action.as_str() {
        "encrypt" => encrypt_text(&key, text),
        "decrypt" => decrypt_text(&key, text),
        _ => println!("Invalid action. Use 'encrypt' or 'decrypt'."),
    }
}

// Encrypts the text and prints the encrypted data in hexadecimal
fn encrypt_text(key: &[u8], text: &str) {
    // Generate random IV
    let mut iv = [0u8; 16];
    rand_bytes(&mut iv).expect("Failed to generate random IV");

    // Create the cipher
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();

    // Encrypt the text
    let mut encrypted_data = cipher.encrypt_vec(text.as_bytes());
    encrypted_data = [&iv[..], &encrypted_data[..]].concat();

    // Print the encrypted data in hexadecimal
    let encrypted_hex = hex::encode(encrypted_data);
    println!("Encrypted data: {}", encrypted_hex);
}

// Decrypts the text and prints the decrypted data
fn decrypt_text(key: &[u8], text: &str) {
    // Parse the IV and ciphertext from the input
    let data = match hex::decode(text) {
        Ok(data) => data,
        Err(_) => {
            println!("Invalid input data. Must be a valid hex-encoded string.");
            return;
        }
    };
    let (iv, ciphertext) = data.split_at(16);

    // Create the cipher
    let cipher = Aes256Cbc::new_from_slices(&key, iv).unwrap();

    // Decrypt the ciphertext
    let decrypted_data = match cipher.decrypt_vec(ciphertext) {
        Ok(data) => data,
        Err(_) => {
            println!("Decryption failed. Ensure the provided password and encrypted data are correct.");
            return;
        }
    };

    // Convert to a string and print
    let decrypted_text = match String::from_utf8(decrypted_data) {
        Ok(text) => text,
        Err(_) => {
            println!("Decrypted data contains invalid UTF-8 characters.");
            return;
        }
    };
    println!("Decrypted text: {}", decrypted_text);
}