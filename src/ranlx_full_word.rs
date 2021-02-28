use rand_core::{
    block::BlockRngCore,
    RngCore,
    SeedableRng,
};

pub struct RanlxFullWord {
    state: [u32; 16],
    offset: usize,
    c: bool,
    generated: usize,
}

impl RanlxFullWord {
    const fn get_state_cycle(&self, pos: usize) -> u32 {
        self.state[(pos + self.offset) % 16]
    }
    
    // TODO revoir
    const S: usize = 3;
    const R: usize = 16;
    const KEPT: usize = Self::R;
    const DISCARD: usize = 71 - Self::KEPT;
    
    /// Does one step by mutating the state
    fn generate_next(&mut self) -> u32 {
        let (sum, overflow) = self.get_state_cycle(16 - RanlxFullWord::S).overflowing_add(self.get_state_cycle(16 - RanlxFullWord::R));
        let (delta_n, overflow_2 ) = sum.overflowing_add(self.c as u32);
        self.c = overflow || overflow_2;
        self.offset = (self.offset + 1) % 16;
        self.state[self.offset] = delta_n;
        delta_n
    }
    
    /// Generate the next u32 taking account the discarded numbers
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

impl RngCore for RanlxFullWord {
    fn next_u32(&mut self) -> u32 {
        self.generate_next_with_discard()
    }
    
    fn next_u64(&mut self) -> u64 {
        let mut x = (self.next_u32() as u64) << 32;
        x |= self.next_u32() as u64;
        x
    }
    
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest)
    }
    
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl BlockRngCore for RanlxFullWord {
    type Item = u32;
    type Results = [u32; 16];
    
    fn generate(&mut self, results: &mut Self::Results) {
        todo!()
    }
}


pub struct SeedRanlxFullWorld {
    seed: [u32; 16],
    c0: bool,
}

/// Seed for [`RanlxFullWorld`]
impl SeedRanlxFullWorld {
    pub const fn new(seed: [u32; 16], c0: bool) -> Self {
        // TODO verify invalide state
        SeedRanlxFullWorld {seed, c0}
    }
    
    pub const fn seed(&self) -> &[u32; 16] {
        &self.seed
    }
    
    pub const fn c0(&self) -> bool {
        self.c0
    }
}

impl Default for SeedRanlxFullWorld {
    fn default () -> Self {
        // TODO revoir
        Self::new([0xe2_98_d6_4a; 16], true)
    }
}

impl AsMut<[u8]> for SeedRanlxFullWorld {
    fn as_mut(&mut self) -> &mut [u8] {
        let ptr = std::ptr::slice_from_raw_parts_mut(self as *mut SeedRanlxFullWorld as *mut u8, 16 * 4 + 1) ;
        unsafe {
            ptr.as_mut().unwrap()
        }
    }
}

impl SeedableRng for RanlxFullWord {
    type Seed = SeedRanlxFullWorld;
    
    fn from_seed(seed: Self::Seed) -> Self {
        Self {
            state: *seed.seed(),
            c: seed.c0(),
            offset: 15,
            generated: 0,
        }
    }
}
