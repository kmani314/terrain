extern crate kiss3d;
extern crate nalgebra as na;
extern crate ncollide3d;

use kiss3d::window::Window;
use kiss3d::light::Light;

use na::{Point3, Vector3, Translation3};

use ncollide3d::procedural::TriMesh;

use crate::mesh;
use crate::noise;

pub struct Color(pub f32, pub f32, pub f32);

pub struct Terrain {
    mesh: TriMesh<f32>, // Abstract the height map away
    size: u32,
    spacing: f32,
    color: Color,
}

impl Terrain {
    pub fn new(color: Color, size: u32, spacing: f32) -> Terrain {
        let mut vec = Vec::new();
        
        // Flat plane
        for i in 0..size {
            for j in 0..size { vec.push(Point3::<f32>::new(spacing*(j as f32), spacing*(i as f32), 0.0)); }
        }
        
        Terrain {
            mesh: mesh::points_to_mesh(vec, size),
            size: size,
            spacing: spacing,
            color: color,
        }
    }
    
    pub fn random(&mut self) {
        self.mesh = mesh::random_mesh(self.size, self.spacing);
    }
    
    pub fn generate_terrain(&mut self) {
        // TODO: Noise and erosion and the rest
        
        let height_map: Vec<Point3<f32>> = noise::get_continuous_noise(self.size, self.spacing);
         
        self.mesh = mesh::points_to_mesh(height_map, self.size);
    }
    
    pub fn render(&mut self) {
        let mut window = Window::new("terrain");
        window.set_background_color(0.529, 0.807, 0.922); // Sky Blue

        let mut mesh = window.add_trimesh(self.mesh.clone(), Vector3::new(1.0, 1.0, 1.0));
         
        mesh.append_translation(&Translation3::new(-self.spacing*(self.size as f32/2.0), -self.spacing*(self.size as f32/2.0), 0.0)); // Center mesh
        
        mesh.set_color(self.color.0, self.color.1, self.color.2);

        window.set_light(Light::StickToCamera);

        while window.render() {}
    }
}
