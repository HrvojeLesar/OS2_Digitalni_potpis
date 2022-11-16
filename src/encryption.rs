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
        encrypt(self.cipher, self.key, self.initialization_vector, data).unwrap()
    }

    pub fn decrypt(&self, encrypted_data: &[u8]) -> Vec<u8> {
        decrypt(
            self.cipher,
            self.key,
            self.initialization_vector,
            encrypted_data,
        )
        .unwrap()
    }

    pub fn encrypt_file(&self, filename: &str) -> Vec<u8> {
        let file = read_file_to_buffer(&filename);
        encrypt(self.cipher, self.key, self.initialization_vector, &file).unwrap()
    }

    pub fn decrypt_file(&self, filename: &str) -> Vec<u8> {
        let file = read_file_to_buffer(&filename);
        encrypt(self.cipher, self.key, self.initialization_vector, &file).unwrap()
    }
}

struct EncryptAsymmetric {
    rsa: Rsa<Private>,
}

impl EncryptAsymmetric {
    pub fn new() -> Self {
        Self {
            rsa: Rsa::generate(2048).unwrap(),
        }
    }

    pub fn new_save_keys() -> Self {
        let rsa = Rsa::generate(2048).unwrap();
        write_file(
            PRIVATE_KEY_FILENAME,
            &rsa.private_key_to_pem().unwrap(),
            false,
        );
        write_file(
            PUBLIC_KEY_FILENAME,
            &rsa.public_key_to_pem().unwrap(),
            false,
        );
        Self {
            rsa: Rsa::generate(2048).unwrap(),
        }
    }

    pub fn from_files(filename: Option<&str>) -> Self {
        let private_key_pem = read_file_to_buffer(filename.unwrap_or(PRIVATE_KEY_FILENAME));
        Self {
            rsa: Rsa::private_key_from_pem(&private_key_pem).unwrap(),
        }
    }

    pub fn private_encrypt(&self, data: &[u8]) -> Vec<u8> {
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
                .private_encrypt(&data[from..to], &mut buf, Padding::PKCS1)
                .unwrap();

            bytes_remaining -= to - from;
            from = to;
            out.append(&mut buf);
            fill_buffer(&mut buf);
        }
        out
    }

    pub fn public_encrypt(&self, data: &[u8]) -> Vec<u8> {
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
                .public_encrypt(&data[from..to], &mut buf, Padding::PKCS1)
                .unwrap();

            bytes_remaining -= to - from;
            from = to;
            out.append(&mut buf);
            fill_buffer(&mut buf);
        }
        out
    }

    pub fn private_decrypt(&self, data: &[u8]) -> Vec<u8> {
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
            let bytes_decrypted = self
                .rsa
                .private_decrypt(&data[from..to], &mut buf, Padding::PKCS1)
                .unwrap();

            bytes_remaining -= to - from;
            from = to;
            out.append(&mut buf[..bytes_decrypted].to_vec());
            fill_buffer(&mut buf);
        }
        out
    }

    pub fn public_decrypt(&self, data: &[u8]) -> Vec<u8> {
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
            let bytes_decrypted = self
                .rsa
                .public_decrypt(&data[from..to], &mut buf, Padding::PKCS1)
                .unwrap();

            bytes_remaining -= to - from;
            from = to;
            out.append(&mut buf[..bytes_decrypted].to_vec());
            fill_buffer(&mut buf);
        }
        out
    }

    pub fn private_encrypt_file(&self, filename: &str) -> Vec<u8> {
        let file = read_file_to_buffer(&filename);
        self.private_encrypt(&file)
    }

    pub fn private_decrypt_file(&self, filename: &str) -> Vec<u8> {
        let file = read_file_to_buffer(&filename);
        self.private_decrypt(&file)
    }

    pub fn public_encrypt_file(&self, filename: &str) -> Vec<u8> {
        let file = read_file_to_buffer(&filename);
        self.public_encrypt(&file)
    }

    pub fn public_decrypt_file(&self, filename: &str) -> Vec<u8> {
        let file = read_file_to_buffer(&filename);
        self.public_decrypt(&file)
    }

    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        let hash = ShaHash::hash(data);
        self.private_encrypt(&hash)
    }

    pub fn sign_file(&self, filename: &str) -> Vec<u8> {
        let data = read_file_to_buffer(filename);
        self.sign(&data)
    }

    pub fn verify_file_signature(&self, filename: &str, signature_filename: &str) -> bool {
        let file = read_file_to_buffer(filename);
        let sig = read_file_to_buffer(signature_filename);
        let hash = ShaHash::hash(&file);
        self.verify(&hash, &sig)
    }

    pub fn verify(&self, hash: &[u8], signature: &[u8]) -> bool {
        let original_data_hash = self.public_decrypt(signature);
        original_data_hash == hash
    }
}

fn fill_buffer(buf: &mut Vec<u8>) {
    if buf.len() < buf.capacity() {
        for _ in buf.len()..buf.capacity() {
            buf.push(0);
        }
    }
}

struct ShaHash;

impl ShaHash {
    pub fn hash(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finish().to_vec()
    }

    pub fn hash_file(filename: &str) -> Vec<u8> {
        let data = read_file_to_buffer(filename);
        ShaHash::hash(&data)
    }
}

#[cfg(test)]
mod tests {
    use openssl::rsa::{Padding, Rsa};

    use crate::{
        encryption::fill_buffer,
        file_manip::{read_file_to_buffer, write_file},
    };

    use super::{EncryptAsymmetric, EncryptSymmetric, ShaHash};

    #[test]
    fn test_encrypt_symmetric() {
        let aes = EncryptSymmetric::default();
        let data = aes.encrypt(b"test");
        assert_eq!(
            data.as_slice(),
            b"\xAC\x54\x67\xA0\xAF\x9E\x17\x1D\xD4\x14\xD9\xB5\x8E\x20\x1C\x2D"
        );
    }

    #[test]
    fn test_decrypt_symmetric() {
        let aes = EncryptSymmetric::default();
        let data = aes.decrypt(b"\xAC\x54\x67\xA0\xAF\x9E\x17\x1D\xD4\x14\xD9\xB5\x8E\x20\x1C\x2D");
        assert_eq!(data, b"test");
    }

    #[test]
    fn enc_test() {
        let file = read_file_to_buffer("testfile");
        let rsa = EncryptAsymmetric::new();
        write_file("enc", &rsa.public_encrypt(&file), false);
        let file = read_file_to_buffer("enc");
        write_file("dec", &rsa.private_decrypt(&file), false);
    }

    #[test]
    fn sign_test() {
        let rsa = EncryptAsymmetric::from_files(Some("privatni_kljuc.txt"));
        let signature = rsa.sign_file("testfile");
        write_file("enc.rs.sig", &signature, false);
    }

    #[test]
    fn verify_test() {
        let rsa = EncryptAsymmetric::from_files(Some("privatni_kljuc.txt"));
        let signature = rsa.sign_file("testfile");
        write_file("enc.rs.sig", &signature, false);
        assert_eq!(rsa.verify_file_signature("testfile", "enc.rs.sign"), true);
    }

    #[test]
    fn enc_test2() {
        let rsa = EncryptAsymmetric::from_files(Some("privatni_kljuc.txt"));
        let data = rsa.private_encrypt(b"data");
        assert_eq!(rsa.public_decrypt(&data), b"data");
    }

    #[test]
    fn sign_and_verify_test() {
        let rsa = EncryptAsymmetric::from_files(Some("privatni_kljuc.txt"));
        let hash = ShaHash::hash(b"data");
        let sig = rsa.sign(b"data");
        let verify = rsa.verify(&hash, &sig);
        assert!(verify);
    }

    #[test]
    fn sign_and_verify_file_test() {
        let rsa = EncryptAsymmetric::from_files(Some("privatni_kljuc.txt"));
        let signature = rsa.sign_file("testfile");
        write_file("sig", &signature, false);
        assert!(rsa.verify_file_signature("testfile", "sig"))
    }
}
