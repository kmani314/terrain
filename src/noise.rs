extern crate rand;
extern crate noise;
extern crate simdnoise;

use noise::{NoiseFn, Seedable, Perlin};
use rand::prelude::*;

// Layers: Number of noise planes layered on top of each other
// Lacunarity: Amount of frequency shift in each higher noise octave
// Amplitude: Z-scaling
// Scale: XY-scaling
pub fn get_noisy_map(size: u32, persistence: f64, layers: u32, lacunarity: f64, amplitude: f64, scale: f64) -> Vec<f32> {
    let mut noise_vec = Vec::new();

    let mut rng = rand::thread_rng();
    let seed = rng.gen::<u32>();
    let perlin = Perlin::new().set_seed(seed);
    
    for y in 0..size {
        for x in 0..size {
            let mut val = 0.0;
            let mut weight = 1.0;
            let mut shift = 1.0;
            let size = size as f64;

            for i in 0..layers {
                val += perlin.get([shift*scale*(x as f64/size), shift*scale*(y as f64/size), 0.0])*weight*amplitude;
                weight *= persistence;
                shift *= lacunarity;
            }

            noise_vec.push(val as f32); 
        }
    }
    noise_vec
}
