use openssl::{
    pkey::Private,
    rsa::{Padding, Rsa},
    sha::Sha256,
    symm::{decrypt, encrypt, Cipher},
};

use crate::{
    file_manip::{read_file_to_buffer, write_file},
    PRIVATE_KEY_FILENAME, PUBLIC_KEY_FILENAME,
};

use anyhow::{anyhow, Result};

const RSA_KEY_LENGTH: u32 = 2048;

pub struct EncryptAes {
    cipher: Cipher,
    key: Vec<u8>,
    initialization_vector: Option<Vec<u8>>,
}

impl EncryptAes {
    pub fn new(cipher: Cipher, key: Vec<u8>, initialization_vector: Option<Vec<u8>>) -> Self {
        Self {
            cipher,
            key,
            initialization_vector,
        }
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        Ok(encrypt(
            self.cipher,
            &self.key,
            self.initialization_vector.as_deref(),
            data,
        )?)
    }

    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>> {
        Ok(decrypt(
            self.cipher,
            &self.key,
            self.initialization_vector.as_deref(),
            encrypted_data,
        )?)
    }

    pub fn encrypt_file(&self, filename: &str) -> Result<Vec<u8>> {
        let file = read_file_to_buffer(filename)?;
        self.encrypt(&file)
    }

    pub fn decrypt_file(&self, filename: &str) -> Result<Vec<u8>> {
        let file = read_file_to_buffer(filename)?;
        self.decrypt(&file)
    }
}

pub struct EncryptRsa {
    rsa: Rsa<Private>,
}

impl EncryptRsa {
    pub fn new_save_keys() -> Result<Self> {
        let rsa = Rsa::generate(RSA_KEY_LENGTH)?;
        write_file(PRIVATE_KEY_FILENAME, &rsa.private_key_to_pem()?, false)?;
        write_file(PUBLIC_KEY_FILENAME, &rsa.public_key_to_pem()?, false)?;
        Ok(Self { rsa })
    }

    pub fn from_files(filename: Option<&str>) -> Result<Self> {
        let private_key_pem = match read_file_to_buffer(filename.unwrap_or(PRIVATE_KEY_FILENAME)) {
            Ok(pem) => pem,
            Err(error) => {
                return Err(anyhow!("Ne postoji par kljuceva | {:?}", error));
            }
        };
        Ok(Self {
            rsa: Rsa::private_key_from_pem(&private_key_pem)?,
        })
    }

    pub fn private_encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(data.len());
        let mut buf = vec![0; self.rsa.size() as usize];
        let mut bytes_remaining = data.len();

        let mut from = 0;
        while bytes_remaining > 0 {
            let to = if bytes_remaining > (self.rsa.size() as usize - 11) {
                from + self.rsa.size() as usize - 11
            } else {
                from + bytes_remaining
            };
            self.rsa
                .private_encrypt(&data[from..to], &mut buf, Padding::PKCS1)?;

            bytes_remaining -= to - from;
            from = to;
            out.append(&mut buf);
            fill_buffer(&mut buf);
        }
        Ok(out)
    }

    pub fn public_encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(data.len());
        let mut buf = vec![0; self.rsa.size() as usize];
        let mut bytes_remaining = data.len();

        let mut from = 0;
        while bytes_remaining > 0 {
            let to = if bytes_remaining > (self.rsa.size() as usize - 11) {
                from + self.rsa.size() as usize - 11
            } else {
                from + bytes_remaining
            };
            self.rsa
                .public_encrypt(&data[from..to], &mut buf, Padding::PKCS1)?;

            bytes_remaining -= to - from;
            from = to;
            out.append(&mut buf);
            fill_buffer(&mut buf);
        }
        Ok(out)
    }

    pub fn private_decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(data.len());
        let mut buf = vec![0; self.rsa.size() as usize];
        let mut bytes_remaining = data.len();

        let mut from = 0;
        while bytes_remaining > 0 {
            let to = if bytes_remaining > (self.rsa.size() as usize) {
                from + self.rsa.size() as usize
            } else {
                from + bytes_remaining
            };
            let bytes_decrypted =
                self.rsa
                    .private_decrypt(&data[from..to], &mut buf, Padding::PKCS1)?;

            bytes_remaining -= to - from;
            from = to;
            out.append(&mut buf[..bytes_decrypted].to_vec());
            fill_buffer(&mut buf);
        }
        Ok(out)
    }

    pub fn public_decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(data.len());
        let mut buf = vec![0; self.rsa.size() as usize];
        let mut bytes_remaining = data.len();

        let mut from = 0;
        while bytes_remaining > 0 {
            let to = if bytes_remaining > (self.rsa.size() as usize) {
                from + self.rsa.size() as usize
            } else {
                from + bytes_remaining
            };
            let bytes_decrypted =
                self.rsa
                    .public_decrypt(&data[from..to], &mut buf, Padding::PKCS1)?;

            bytes_remaining -= to - from;
            from = to;
            out.append(&mut buf[..bytes_decrypted].to_vec());
            fill_buffer(&mut buf);
        }
        Ok(out)
    }

    pub fn private_encrypt_file(&self, filename: &str) -> Result<Vec<u8>> {
        let file = read_file_to_buffer(filename)?;
        self.private_encrypt(&file)
    }

    pub fn private_decrypt_file(&self, filename: &str) -> Result<Vec<u8>> {
        let file = read_file_to_buffer(filename)?;
        self.private_decrypt(&file)
    }

    pub fn public_encrypt_file(&self, filename: &str) -> Result<Vec<u8>> {
        let file = read_file_to_buffer(filename)?;
        self.public_encrypt(&file)
    }

    pub fn public_decrypt_file(&self, filename: &str) -> Result<Vec<u8>> {
        let file = read_file_to_buffer(filename)?;
        self.public_decrypt(&file)
    }

    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        let hash = ShaHash::hash(data)?;
        self.private_encrypt(&hash)
    }

    pub fn sign_file(&self, filename: &str) -> Result<Vec<u8>> {
        let data = read_file_to_buffer(filename)?;
        self.sign(&data)
    }

    pub fn verify_file_signature(&self, filename: &str, signature_filename: &str) -> Result<bool> {
        let file = read_file_to_buffer(filename)?;
        let sig = read_file_to_buffer(signature_filename)?;
        let hash = ShaHash::hash(&file)?;
        self.verify(&hash, &sig)
    }

    pub fn verify(&self, hash: &[u8], signature: &[u8]) -> Result<bool> {
        let original_data_hash = self.public_decrypt(signature)?;
        Ok(original_data_hash == hash)
    }
}

fn fill_buffer(buf: &mut Vec<u8>) {
    if buf.len() < buf.capacity() {
        for _ in buf.len()..buf.capacity() {
            buf.push(0);
        }
    }
}

pub struct ShaHash;

impl ShaHash {
    pub fn hash(data: &[u8]) -> Result<Vec<u8>> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        Ok(hasher.finish().to_vec())
    }

    pub fn hash_file(filename: &str) -> Result<Vec<u8>> {
        let data = read_file_to_buffer(filename)?;
        ShaHash::hash(&data)
    }
}
