# Secret Keeper

A simple and secure command-line tool to protect your text by encrypting and decrypting it using the robust AES-256 algorithm. Built with Rust and GPT4.

### Installation
```
cargo install secretkeeper
```

### Usage

```
sk encrypt <text> <password>
sk decrypt <text> <password>
```

**Ensure you keep the password safe; without it, you will not be able to recover the original text from the encrypted data.**

This Rust-based command-line tool provides a simple yet secure way to encrypt and decrypt text using the AES-256 algorithm in CBC mode with PKCS7 padding. By taking an action (encrypt or decrypt), the input text, and a password, it offers an effective method to protect sensitive information. The encryption process involves hashing the provided password to create a 32-byte key, generating a random initialization vector (IV), and then encrypting the input text. The output is presented as a hexadecimal string, which combines the IV and the encrypted text, ensuring security during data storage or transmission.

The decryption process requires the same password that was used for encryption, ensuring that only users with the correct password can access the original content. The tool first separates the IV and the encrypted text from the provided input data. Then, it creates a cipher using the hashed password and the IV to decrypt the text. If the decryption is successful, the tool outputs the decrypted text, allowing users to recover their original information. To ensure the tool's usability and efficiency, the encryption and decryption functionalities have been separated into two distinct functions, improving the overall structure and maintainability of the code.

Overall, this encryption tool offers a straightforward and secure solution for users looking to protect their sensitive data with a widely-used cryptographic algorithm. The use of AES-256 in CBC mode, along with PKCS7 padding and a random IV, provides strong security against potential attacks. The command-line interface makes the tool easy to use and integrate into various workflows, while the code enhances the maintainability for future updates or customizations.