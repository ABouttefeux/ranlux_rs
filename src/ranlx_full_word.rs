use rand_core::{
    block::BlockRngCore,
    RngCore,
    SeedableRng,
};
#[cfg(feature = "serde-serialize")]
use serde::{Serialize, Deserialize};

// AWC(2^32, 3, 16)[16, 71]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct RanlxFullWord32 {
    state: [u32; Self::SIZE],
    offset: usize,
    c: bool,
    generated: usize,
}

impl RanlxFullWord32 {
    
    // TODO revoir
    const SIZE: usize = 16;
    const S: usize = 3;
    const R: usize = Self::SIZE;
    const KEPT: usize = Self::R;
    const DISCARD: usize = 71 - Self::KEPT;
    
    const fn get_state_cycle(&self, pos: usize) -> u32 {
        self.state[(pos + self.offset) % Self::SIZE]
    }
    
    /// Does one step by mutating the state
    #[inline]
    fn generate_next(&mut self) -> u32 {
        let (sum, overflow) = self.get_state_cycle(Self::SIZE - Self::S).overflowing_add(self.get_state_cycle(Self::SIZE - Self::R));
        let (delta_n, overflow_2 ) = sum.overflowing_add(self.c as u32);
        self.c = overflow || overflow_2;
        self.offset = (self.offset + 1) % Self::SIZE;
        self.state[self.offset] = delta_n;
        delta_n
    }
    
    /// Generate the next u32 taking account the discarded numbers
    #[inline]
    fn generate_next_with_discard(&mut self) -> u32 {
        if self.generated > Self::KEPT {
            for _ in 0..Self::DISCARD {
                self.generate_next();
            }
            self.generated = 0;
        }
        self.generated += 1;
        self.generate_next()
    }
}

impl RngCore for RanlxFullWord32 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.generate_next_with_discard()
    }
    
    #[inline]
    fn next_u64(&mut self) -> u64 {
        let x = (self.next_u32() as u64) << 32;
        x | self.next_u32() as u64
    }
    
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest)
    }
    
    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl BlockRngCore for RanlxFullWord32 {
    type Item = u32;
    type Results = [u32; Self::KEPT];
    
    fn generate(&mut self, results: &mut Self::Results) {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct SeedRanlxFullWorld32 {
    seed: [u32; RanlxFullWord32::SIZE],
    c0: bool,
}

/// Seed for [`RanlxFullWorld`]
impl SeedRanlxFullWorld32 {
    pub const fn new(seed: [u32; RanlxFullWord32::SIZE], c0: bool) -> Self {
        // TODO verify invalide state
        Self {seed, c0}
    }
    
    pub const fn seed(&self) -> &[u32; RanlxFullWord32::SIZE] {
        &self.seed
    }
    
    pub const fn c0(&self) -> bool {
        self.c0
    }
}

impl Default for SeedRanlxFullWorld32 {
    fn default () -> Self {
        Self::new([0; RanlxFullWord32::SIZE], false)
    }
}

/// seeding using [Ranlxs::seed_from_u64] is done using [`xoshiro::SplitMix64`]
impl AsMut<[u8]> for SeedRanlxFullWorld32 {
    fn as_mut(&mut self) -> &mut [u8] {
        let ptr = core::ptr::slice_from_raw_parts_mut(self as *mut Self as *mut u8, RanlxFullWord32::SIZE * 4 + 1) ;
        unsafe {
            ptr.as_mut().unwrap()
        }
    }
}

impl SeedableRng for RanlxFullWord32 {
    type Seed = SeedRanlxFullWorld32;
    
    fn from_seed(seed: Self::Seed) -> Self {
        Self {
            state: *seed.seed(),
            c: seed.c0(),
            offset: Self::SIZE - 1,
            generated: 0,
        }
    }
    
    fn seed_from_u64(seed: u64) -> Self {
        let mut rng = rand_xoshiro::SplitMix64::seed_from_u64(seed);
        Self::from_rng(&mut rng).unwrap()
    }
}
