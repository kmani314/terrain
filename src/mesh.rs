extern crate nalgebra as na;
extern crate ncollide3d;

use ncollide3d::procedural::{TriMesh, IndexBuffer};
use na::Point3;

// Generates indecies and mesh from height map
pub fn points_to_mesh(height_map: Vec<f32>, size: u32, density: u32) -> TriMesh<f32> {
    let mut points = Vec::new();
    let mut indecies = Vec::new();

    let incr = 1.0/density as f32;

    let mut i_val = 0.0;
    for i in (0..size*density) {
        let mut j_val = 0.0;
        for j in (0..size*density) {
            // Compute points
            points.push(Point3::<f32>::new(j_val, i_val, height_map[(i*size + j) as usize]));

            // Compute triangles
            let row_len = size*density;
            if i != row_len - 1 && j != row_len - 1 {
                indecies.push(Point3::<u32>::new(i*row_len + j + 1, (i + 1)*row_len + j, (i + 1)*row_len + j + 1));
                indecies.push(Point3::<u32>::new(i*row_len + j + 1, i*row_len + j, (i + 1)*row_len + j));
            }
            j_val += incr;
        }
        i_val += incr;
    }
    //println!("{}", indecies.len());
    TriMesh::new(points, None, None, Some(IndexBuffer::Unified(indecies)))
}
