use rand_core::{
    block::BlockRngCore,
    RngCore,
    SeedableRng,
};

pub struct Ranlxs {
    state: [u32; 24],
    offset: usize,
    c: bool,
    generated: usize,
    extracted_bytes: usize,
}

/// ranlxs algorithm, i.e. the SWB(2^24, 10, 24)[24, 223]
impl Ranlxs {
    const fn get_state_cycle(&self, pos: usize) -> u32 {
        // The mask is here to guarentee that we gives a u24
        self.state[(pos + self.offset) % 32] & 0x00_ff_ff_ff
    }
    
    #[allow(clippy::cast_possible_truncation)]
    /// return the next byte of data
    fn get_next_byte(&mut self) -> u8 {
        let extracted = self.extracted_bytes;
        if extracted == 0 {
            self.generate_next_with_discard();
        }
        self.extracted_bytes = (self.extracted_bytes + 1) % 3;
        let number = self.get_state_cycle(0);
        // The truncation is wanted as we want to only extracts the used byte
        (number >> (16 - extracted * 8)) as u8
    }
    
    // TODO revoir
    const R: usize = 24;
    const S: usize = 10;
    const KEPT: usize = Self::R;
    const DISCARD: usize = 223 - Self::KEPT;
    
    /// Does one step by mutating the state
    fn generate_next(&mut self) -> u32 {
        let (dif, overflow) = self.get_state_cycle(24 - Self::S).overflowing_sub(self.get_state_cycle(24 - Self::R));
        let (delta_n, overflow_2) = dif.overflowing_sub(self.c as u32);
        self.c = overflow || overflow_2;
        self.offset = (self.offset + 1) % 32;
        self.state[self.offset] = delta_n;
        delta_n
    }
    
    /// Generate the next u24 taking account the discarded numbers
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

impl RngCore for Ranlxs {
    fn next_u32(&mut self) -> u32 {
        let mut bytes = [0_u8; 4];
        for el in bytes.iter_mut() {
            *el = self.get_next_byte();
        }
        u32::from_le_bytes(bytes)
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

impl BlockRngCore for Ranlxs {
    type Item = u64;
    type Results = [u64; 24];
    
    fn generate(&mut self, results: &mut Self::Results) {
        todo!()
    }
}

/// Seed for [`Ranlxs`]
pub struct SeedRanlxs {
    seed: [u8; 24 * 3],
    c0: bool,
}

impl SeedRanlxs {
    pub const fn new(seed: [u8; 24 * 3], c0: bool) -> Self {
        // TODO verify invalide state
        SeedRanlxs {seed, c0}
    }
    
    pub const fn seed(&self) -> &[u8; 24 * 3] {
        &self.seed
    }
    
    pub const fn c0(&self) -> bool {
        self.c0
    }
}

impl Default for SeedRanlxs {
    fn default () -> Self {
        // TODO revoir
        Self::new([0x4a; 24 * 3], true)
    }
}

impl AsMut<[u8]> for SeedRanlxs {
    fn as_mut(&mut self) -> &mut [u8] {
        let ptr = std::ptr::slice_from_raw_parts_mut(self as *mut SeedRanlxs as *mut u8, 24 * 3 + 1) ;
        unsafe {
            ptr.as_mut().unwrap()
        }
    }
}

impl SeedableRng for Ranlxs {
    type Seed = SeedRanlxs;
    
    fn from_seed(seed: Self::Seed) -> Self {
        let mut state = [0_u32; 24];
        for (i, el) in state.iter_mut().enumerate() {
            let mut slice_bytes = [0_u8; 4];
            for (j, byte) in slice_bytes.iter_mut().enumerate().take(3) {
                *byte = seed.seed()[i * 3 + j];
            }
            *el = u32::from_be_bytes(slice_bytes);
        }
        Self {
            state,
            c: seed.c0(),
            offset: 23,
            generated: 0,
            extracted_bytes: 0
        }
    }
}
