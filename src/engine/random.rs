use rand::prelude::*;

pub struct RandomApi;

impl RandomApi {
    // TODO: Add provably fair randomness
    pub fn rand_usize(min: usize, max: usize) -> usize {
        let x: usize = rand::random_range(min..max);
        x
    }
    pub fn rand_int(min: u32, max: u32) -> u32 {
        let x: u32 = rand::random_range(min..max);
        x
    }
}
