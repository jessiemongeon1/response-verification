use rand::{CryptoRng, Error, Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

/// Byte length of the seed type used in [`ReproducibleRng`].
const SEED_LEN: usize = 32;

/// Provides a seeded RNG, where the randomly chosen seed is printed on standard output.
pub fn reproducible_rng() -> ReproducibleRng {
    ReproducibleRng::new()
}

/// Wraps the logic of [`reproducible_rng`] into a separate struct.
///
/// This is needed when [`reproducible_rng`] cannot be used because its
/// return type `impl Rng + CryptoRng` can only be used as function parameter
/// or as return type
/// (See [impl trait type](https://doc.rust-lang.org/reference/types/impl-trait.html)).
pub struct ReproducibleRng {
    rng: ChaCha20Rng,
    seed: [u8; SEED_LEN],
}

impl ReproducibleRng {
    /// Randomly generates a seed and prints it to `stdout`.
    pub fn new() -> Self {
        let mut seed = [0u8; SEED_LEN];
        rand::thread_rng().fill(&mut seed);
        let rng = Self::from_seed_internal(seed);
        println!("{rng:?}");
        rng
    }

    fn from_seed_internal(seed: [u8; SEED_LEN]) -> Self {
        let rng = ChaCha20Rng::from_seed(seed);
        Self { rng, seed }
    }
}

impl std::fmt::Debug for ReproducibleRng {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Copy the seed below to reproduce the failed test.\n
    let seed: [u8; 32] = {:?};",
            self.seed
        )
    }
}

impl Default for ReproducibleRng {
    fn default() -> Self {
        Self::new()
    }
}

impl RngCore for ReproducibleRng {
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.rng.try_fill_bytes(dest)
    }
}

impl CryptoRng for ReproducibleRng {}
