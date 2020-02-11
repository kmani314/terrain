extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

use na::{Point3};

use kiss3d::resource::Mesh;
use rand::Rng;

// Generates random mesh from a matrix of size [x, y]
pub fn random_mesh(x: usize, y: usize) -> Mesh {
    // TODO

    let mut map = Vec::new();
    let rng = rand::thread_rng();

    for i in 0..y {
        for j in 0..x {
            map.push(Point3::new(j as f32, i as f32, rng.gen::<f32>()));
        }
    }

    points_to_mesh(map)
}

// Generates the faces for a triangle mesh from a set of points
pub fn points_to_mesh(points: Vec<Point3<f32>>) -> Mesh {
    // TODO
}
