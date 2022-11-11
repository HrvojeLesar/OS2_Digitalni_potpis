use rand::{rngs::ThreadRng, Rng};

struct Keygen {
    rng: ThreadRng,
}

impl Default for Keygen {
    fn default() -> Self {
        Self {
            rng: ThreadRng::default(),
        }
    }
}

impl Keygen {
    pub fn generate_128bit_key(&mut self) -> [u8; 16] {
        self.rng.gen()
    }

    pub fn generate_192bit_key(&mut self) -> [u8; 24] {
        self.rng.gen()
    }

    pub fn generate_256bit_key(&mut self) -> [u8; 32] {
        self.rng.gen()
    }
}

#[cfg(test)]
mod tests {
    use super::Keygen;

    #[test]
    fn test_generate() {
        let mut keygen = Keygen::default();
        let key_128 = keygen.generate_128bit_key();
        let key_192 = keygen.generate_192bit_key();
        let key_256 = keygen.generate_256bit_key();

        assert_eq!(key_128.len(), 16);
        assert_eq!(key_192.len(), 24);
        assert_eq!(key_256.len(), 32);
    }
}
