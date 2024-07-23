// SPDX-FileCopyrightText: © 2024 Christopher Woods <Christopher.Woods@bristol.ac.uk>
// SPDX-License-Identifier: MIT

use anyhow::{bail, Result};
use orion::aead;
use secrecy::{CloneableSecret, DebugSecret, Secret, SerializableSecret, Zeroize};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::serde_as;
use std::{fmt, io::Read, str, vec};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("{0}")]
struct CryptoError(String);

#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub data: vec::Vec<u8>,
    pub version: u8,
}

impl fmt::Debug for EncryptedData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EncryptedData {{ data: [REDACTED] length {} bytes, version: {} }}",
            self.data.len(),
            self.version
        )
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub data: vec::Vec<u8>,
}

impl Zeroize for Key {
    fn zeroize(&mut self) {
        self.data.zeroize();
    }
}

/// Permits cloning, Debug printing as [[REDACTED]] and serialising
impl CloneableSecret for Key {}
impl DebugSecret for Key {}
impl SerializableSecret for Key {}

pub type SecretKey = Secret<Key>;

impl Key {
    ///
    /// Generate a new secret key.
    /// This will return a new secret key that can be used to encrypt and decrypt messages.
    ///
    /// # Returns
    ///
    /// The secret key.
    ///
    /// # Example
    ///
    /// ```
    /// use paddington::crypto::Key;
    ///
    /// let key = Key::generate();
    /// ```
    pub fn generate() -> SecretKey {
        Key {
            data: aead::SecretKey::default().unprotected_as_bytes().to_vec(),
        }
        .into()
    }

    ///
    /// Encrypt the passed data with this key.
    /// This will return the encrypted data as a struct
    /// that can be serialised and deserialised by serde.
    /// Note that the data must be serialisable and deserialisable
    /// by serde.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to encrypt.
    ///
    /// # Returns
    ///
    /// The encrypted data.
    ///
    /// # Example
    ///
    /// ```
    /// use paddington::crypto::{Key, SecretKey};
    ///
    /// let key = Key::generate();
    ///
    /// let encrypted_data = key.expose_secret().encrypt("Hello, World!".to_string());
    /// ```
    pub fn encrypt<T>(&self, data: T) -> Result<EncryptedData>
    where
        T: Serialize,
    {
        let orion_key = aead::SecretKey::from_slice(&self.data)?;
        let json_data = serde_json::to_string(&data)?;
        println!("data: {:?}", json_data);

        Ok(EncryptedData {
            data: aead::seal(&orion_key, json_data.as_bytes())?,
            version: 1,
        })
    }

    ///
    /// Decrypt the passed data with this key.
    /// This will return the decrypted data.
    ///
    /// Arguments
    ///
    /// * `data` - The data to decrypt.
    ///
    /// Returns
    ///
    /// The decrypted data.
    ///
    /// Example
    ///
    /// ```
    /// use paddington::crypto::{Key, SecretKey};
    ///
    /// let key = Key::generate();
    ///
    /// let encrypted_data = key.expose_secret().encrypt("Hello, World!".to_string());
    /// let decrypted_data = key.expose_secret().decrypt(&encrypted_data).unwrap();
    ///
    /// assert_eq!(decrypted_data, "Hello, World!".to_string());
    /// ```
    pub fn decrypt<T>(&self, data: &EncryptedData) -> Result<T>
    where
        T: DeserializeOwned,
    {
        if data.version != 1 {
            bail!(CryptoError(format!(
                "Only version 1 is supported. This is version {:?}",
                data.version
            )));
        }

        let orion_key = aead::SecretKey::from_slice(&self.data)?;
        let decrypted_data = aead::open(&orion_key, &data.data)?;

        let decrypted_string: String = String::from_utf8(decrypted_data)?;

        println!("decrypted_string: {:?}", decrypted_string);

        let obj: T = serde_json::from_str(&decrypted_string)?;

        Ok(obj)
    }
}
