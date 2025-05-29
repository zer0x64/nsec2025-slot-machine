use rand_core::{impls, RngCore, SeedableRng};

pub struct SimpleLfsr(u64, usize);

impl SeedableRng for SimpleLfsr {
    type Seed = [u8; 8];

    fn from_seed(seed: Self::Seed) -> Self {
        Self(u64::from_ne_bytes(seed), 0)
    }
}

impl RngCore for SimpleLfsr {
    fn next_u32(&mut self) -> u32 {
        if self.1 >= 5 {
            self.1 = 0;

            // Masks so it's 40 bits instead of 64
            self.0 ^= self.0 << 10;
            self.0 &= 0xFFFFFFFFFF;

            self.0 ^= self.0 >> 26;
            self.0 &= 0xFFFFFFFFFF;
        }

        // Mask with a random constant for added difficulty
        let result = (self.0 ^ 0xf267bcb3b2) >> (self.1 * 8);

        self.1 += 1;
        result as u32
    }

    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest);
    }
}
