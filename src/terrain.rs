extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

use kiss3d::window::Window;
use kiss3d::light::Light;

use na::{Translation3, Vector3};

use crate::mesh;
use crate::noise;

use rand::prelude::*;

pub struct Color(pub f32, pub f32, pub f32);

pub struct Terrain {
    height_map: Vec<f32>, // Abstract the height map away
    size: u32,
    density: u32,
    color: Color,
}

impl Terrain {
    pub fn new(color: Color, size: u32, density: u32) -> Terrain {
        Terrain {
            height_map: Vec::new(),
            size: size,
            density: density,
            color: color,
        }
    }
    
    pub fn random(&mut self) {
        let mut rng = rand::thread_rng();
        let mut map = Vec::new();

        for i in 0..self.size*self.density {
            for j in 0..self.size*self.density {
                map.push(rng.gen::<f32>());
            }
        }
        self.height_map = map;
    }
    
    pub fn generate_terrain(&mut self) {
        // TODO: Noise and erosion and the rest
    }
    
    pub fn render(&mut self) {
        let mut window = Window::new("terrain");
        window.set_background_color(0.529, 0.807, 0.922); // Sky Blue

        let mut mesh = window.add_trimesh(mesh::points_to_mesh(self.height_map.clone(), self.size, self.density), Vector3::new(1.0, 1.0, 1.0));
        mesh.append_translation(&Translation3::new(-(self.size as f32/2.0), -(self.size as f32/2.0), 0.0)); // Center mesh
        mesh.set_color(self.color.0, self.color.1, self.color.2);
        window.set_light(Light::StickToCamera);
        while window.render() {}
    }
}
