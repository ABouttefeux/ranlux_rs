use rand_core::{
    block::BlockRngCore,
    RngCore,
    SeedableRng,
};

pub struct Ranlxd {
    
}


impl RngCore for Ranlxd {
    fn next_u32(&mut self) -> u32 {
        todo!()
    }
    
    fn next_u64(&mut self) -> u64 {
        todo!()
    }
    
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        todo!()
    }
    
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl BlockRngCore for Ranlxd {
    type Item = f64;
    type Results = [f64; 24];
    
    fn generate(&mut self, results: &mut Self::Results) {
        todo!()
    }
}

/// seeding using [Ranlxs::seed_from_u64] is done using [`xoshiro::SplitMix64`]
impl SeedableRng for Ranlxd {
    type Seed = [u8; 4];
    
    fn from_seed(seed: Self::Seed) -> Self {
        todo!()
    }
    
    fn seed_from_u64(seed: u64) -> Self {
        let mut rng = rand_xoshiro::SplitMix64::seed_from_u64(seed);
        Self::from_rng(&mut rng).unwrap()
    }
}
