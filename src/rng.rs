// basic PCG

use std::time::{SystemTime, UNIX_EPOCH};

pub struct RNG {
    state: u64,
    increment: u64
}

impl RNG {
    pub fn new() -> RNG {
        RNG {
            state:     0x853c49e6748fea9b,
            increment: 0xda3e39cb94b95bdb
        }
    }

    pub fn srand(&mut self, initstate: u64, initseq: u64) {
        self.state = 0;
        self.increment = (initseq << 1) | 1;
        self.random();
        self.state += initstate;
        self.random();
    }

    pub fn seed_from_current_time(&mut self) {
        let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        self.srand(t.as_nanos() as u64, 5)
    }

    pub fn random(&mut self) -> u32 {
        let oldstate = self.state;
        self.state = oldstate * 6364136223846793005 + self.increment;
        let xorshifted = ((oldstate << 18) ^ oldstate) >> 27;
        let rot = oldstate >> 59;
        if rot != 0 {
            return ((xorshifted >> rot) | (xorshifted << (32 - rot))) as u32;
        } else {
            return xorshifted as u32;
        }
    }

    pub fn random_index(&mut self, bound: usize) -> usize {
        let mut negbound = bound as i32;
        negbound = -negbound;

        let threshold = (negbound as u32) % (bound as u32);

        loop {
            let r = self.random();
            if r >= threshold {
                return (r % (bound as u32)) as usize;
            }
        }
    }

    pub fn shuffle<T>(&mut self, list: &mut Vec<T>) {
        for i in (1..list.len()).rev() {
            list.swap(i, self.random_index(i + 1));
        }
    }
}
