extern crate rand;
extern crate noise;

use noise::{NoiseFn, OpenSimplex, Seedable, Perlin};
use rand::prelude::*;

pub fn get_noisy_map(size: u32, persistence: f64, lower: f64, upper: f64, layers: u32, lacunarity: f64) -> Vec<f32> {
    let perlin = Perlin::new();
    let mut rng = rand::thread_rng();
    let seed = rng.gen::<u32>();
    println!("{}", seed);
    perlin.set_seed(seed);
    let mut noise_vec = Vec::new();
    
    let scale = 10.0;
    for y in 0..size {
        for x in 0..size {
            let mut val = 0.0;
            let mut weight = 1.0;
            let mut shift = 1.0;
            let size = size as f64;

            for i in 0..layers {
                val += perlin.get([shift*(x as f64/size), shift*(y as f64/size), 0.0])*weight;
                weight *= persistence;
                shift *= lacunarity;
            }
            noise_vec.push(val as f32*scale); 
        }
    }
    noise_vec
}
