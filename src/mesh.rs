extern crate nalgebra as na;
extern crate ncollide3d;

use ncollide3d::procedural::{TriMesh, IndexBuffer};
use na::Point3;

// Generates indecies and mesh from height map
pub fn points_to_mesh(height_map: &[f32], size: u32, elevation: f32) -> TriMesh<f32> {
    let mut points = Vec::new();
    let mut indecies = Vec::new();

    for i in 0..size {
        for j in 0..size {
            points.push(Point3::<f32>::new(j as f32, height_map[(i*size + j) as usize]*elevation, i as f32)); // ArcBall uses y as up :(

            if i != size - 1 && j != size - 1 {
                indecies.push(Point3::<u32>::new(i*size+ j + 1, (i + 1)*size + j, (i + 1)*size + j + 1));
                indecies.push(Point3::<u32>::new(i*size+ j + 1, i*size+ j, (i + 1)*size + j));
            }
        }
    }

    TriMesh::new(points, None, None, Some(IndexBuffer::Unified(indecies)))
}
