extern crate nalgebra as na;
extern crate rand;
extern crate ncollide3d;

use ncollide3d::procedural::{TriMesh, IndexBuffer};
use na::Point3;

use rand::Rng;

// Generates random mesh of size size 
pub fn random_mesh(size: u32) -> TriMesh<f32> {
    let mut map = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..size {
        for j in 0..size {
            map.push(Point3::new(j as f32, i as f32, rng.gen::<f32>()*5.0));
        }
    }

    points_to_mesh(map, size)
}

// Generates the faces for a triangle mesh from a set of points
pub fn points_to_mesh(points: Vec<Point3<f32>>, size: u32) -> TriMesh<f32> {
    let mut indecies = Vec::new();

    for i in 0..size {
        for j in 0..size {
            // Compute nearest two vertices in the grid to form triangles

            if i != size - 1 && j != size - 1 {
                indecies.push(Point3::<u32>::new(i*size + j + 1, (i + 1)*size + j, (i + 1)*size + j + 1));
                indecies.push(Point3::<u32>::new(i*size + j + 1, i*size + j, (i + 1)*size + j));
            }
        }
    }
    
    TriMesh::new(points, None, None, Some(IndexBuffer::Unified(indecies)))
}
