pub struct LcgRng {
    next: u32,
}

impl LcgRng {
    pub fn new(seed: u32) -> Self {
        LcgRng { next: seed }
    }

    pub fn rand(&mut self) -> i32 {
        // Perform the LCG calculation.
        // Rust's default integer overflow behavior is wrapping in release builds,
        // which matches C's unsigned integer behavior for these operations.
        self.next = self.next.wrapping_mul(1103515245).wrapping_add(12345);

        ((self.next / 65536) % 32768) as i32
    }

    pub fn srand(&mut self, seed: u32) {
        self.next = seed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization_and_first_values() {
        let mut rng = LcgRng::new(1); // Seed 1
        assert_eq!(rng.rand(), 16838);
        assert_eq!(rng.rand(), 5758);
        assert_eq!(rng.rand(), 10113);

        let mut rng = LcgRng::new(2); // Seed 2
        assert_ne!(rng.rand(), 16838);
        assert_ne!(rng.rand(), 5758);
        assert_ne!(rng.rand(), 10113);
    }

        #[test]
    fn test_reproducibility() {
        let seed = 12345;

        // Run 1: Generate a sequence
        let mut rng1 = LcgRng::new(seed);
        let mut seq1 = Vec::new();
        for _ in 0..10 {
            seq1.push(rng1.rand());
        }

        // Run 2: Generate another sequence with the exact same seed
        let mut rng2 = LcgRng::new(seed);
        let mut seq2 = Vec::new();
        for _ in 0..10 {
            seq2.push(rng2.rand());
        }

        // Assert that the two sequences are identical
        assert_eq!(seq1, seq2);
    }
    #[test]
    fn test_srand_reproducibility() {
        let initial_seed = 54321;
        let reseed_value = 98765;

        let mut rng = LcgRng::new(initial_seed); // Start with initial_seed
        let val1_1 = rng.rand();
        let val1_2 = rng.rand();

        rng.srand(reseed_value); // Reseed to a different value
        let val2_1 = rng.rand();
        let val2_2 = rng.rand();

        rng.srand(initial_seed); // Reseed back to the initial_seed
        let val3_1 = rng.rand();
        let val3_2 = rng.rand();

        // After reseeding with `initial_seed`, the sequence should restart from there.
        // So, val3_1 should be equal to val1_1, and val3_2 to val1_2.
        assert_eq!(val1_1, val3_1, "First value after re-seeding with initial_seed should match.");
        assert_eq!(val1_2, val3_2, "Second value after re-seeding with initial_seed should match.");

        // The values after re-seeding with `reseed_value` should be different from initial_seed's sequence.
        assert_ne!(val1_1, val2_1, "Values from different seeds should not match.");
    }

    #[test]
    fn test_different_seeds_produce_different_sequences() {
        let mut rng_a = LcgRng::new(10);
        let mut rng_b = LcgRng::new(20); // Different seed

        let mut seq_a = Vec::new();
        let mut seq_b = Vec::new();

        for _ in 0..5 {
            seq_a.push(rng_a.rand());
            seq_b.push(rng_b.rand());
        }

        assert_ne!(seq_a, seq_b, "Sequences from different seeds should not be equal.");
    }
}