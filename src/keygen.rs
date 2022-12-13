use rand::{rngs::ThreadRng, Rng};

#[derive(Default)]
pub struct Keygen {
    rng: ThreadRng,
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
