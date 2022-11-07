use openssl::symm::{decrypt, encrypt, Cipher};

struct EncryptSymmetric<'a> {
    cipher: Cipher,
    key: &'a [u8],
    initialization_vector: Option<&'a [u8]>,
}

impl<'a> Default for EncryptSymmetric<'a> {
    fn default() -> Self {
        Self {
            cipher: Cipher::aes_128_cbc(),
            key: b"secure_128bitkey",
            initialization_vector: None,
        }
    }
}

impl<'a> EncryptSymmetric<'a> {
    pub fn new(cipher: Cipher, key: &'a [u8], initialization_vector: Option<&'a [u8]>) -> Self {
        Self {
            cipher,
            key,
            initialization_vector,
        }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        encrypt(self.cipher, self.key, None, data).unwrap()
    }

    pub fn decrypt(&self, encrypted_data: &[u8]) -> Vec<u8> {
        decrypt(self.cipher, self.key, None, encrypted_data).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::EncryptSymmetric;

    #[test]
    fn test_encrypt_symmetric() {
        let aes = EncryptSymmetric::default();
        let data = aes.encrypt(b"test");
        assert_eq!(data.as_slice(), b"\xAC\x54\x67\xA0\xAF\x9E\x17\x1D\xD4\x14\xD9\xB5\x8E\x20\x1C\x2D");
    }

    #[test]
    fn test_decrypt_symmetric() {
        let aes = EncryptSymmetric::default();
        let data = aes.decrypt(b"\xAC\x54\x67\xA0\xAF\x9E\x17\x1D\xD4\x14\xD9\xB5\x8E\x20\x1C\x2D");
        assert_eq!(data, b"test");
    }
}
