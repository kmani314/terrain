extern crate noise;
extern crate nalgebra as na;

use na::{Point2, Point3};

use noise::{Perlin, Seedable};
use noise::NoiseFn;

pub fn get_continuous_noise(size: u32, spacing: f32) -> Vec<Point3<f32>> {
    let perlin = Perlin::new();

    let mut noise_vec = Vec::new();
    
    // Rewrite this
    for i in 0..size {
        for j in 0..size {
            noise_vec.push(Point3::<f32>::new(spacing*(j as f32), spacing*(i as f32), 0.1*(perlin.get([(j as f64)*0.1, (i as f64)*0.1]) as f32)));
        }
    }
    noise_vec
}
