use std::num::Wrapping;

// An unremarkable odd number
const ODD: Wrapping<u64> = Wrapping(0x9f32e1cbc5e1374b);

pub(crate) struct MSQW {
    reservoir: u64,
    level: u8,
    accumulator: Wrapping<u64>,
    state: Wrapping<u64>,
}

impl MSQW {
    pub fn new() -> Self { 
        Self { accumulator: Wrapping(0), state: Wrapping(0), reservoir: 0, level: 0 }
    }

    pub fn update(&mut self) -> u32 {
        self.accumulator += ODD;
        self.state += self.accumulator;
        self.state = self.state * self.state;
        self.state = Wrapping(self.state.0.rotate_left(32));

        return self.state.0 as u32;
    }

    pub fn update_xor(&mut self) -> u32 {
        self.accumulator += ODD;
        self.state ^= self.accumulator;
        self.state = self.state * self.state;
        self.state = Wrapping(self.state.0.rotate_left(32));

        return self.state.0 as u32;
    }

    pub fn mix(&mut self, randomness: u64) {
        self.state ^= Wrapping(randomness);
        self.update_xor();
    }

    pub fn add(&mut self, bit: bool) {
        self.level += 1;

        self.reservoir <<= 1;
        if bit {
            self.reservoir |= 1;
        }

        if self.level == 64 {
            self.mix(self.reservoir);
            self.level = 0;
        }
    }
}
