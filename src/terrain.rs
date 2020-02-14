extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::ArcBall;

use na::{Point3, Translation3, Vector3, UnitQuaternion};

use crate::mesh;
use crate::noise;

use rand::prelude::*;

pub struct Color(pub f32, pub f32, pub f32);

pub struct Terrain {
    height_map: Vec<f32>, // Abstract the height map away
    size: u32,
    scale: f32,
    color: Color,
}

impl Terrain {
    pub fn new(color: Color, size: u32, scale: f32) -> Terrain {
        Terrain {
            height_map: Vec::new(),
            size: size,
            scale: scale,
            color: color,
        }
    }
    
    pub fn random(&mut self) {
        let mut rng = rand::thread_rng();
        let mut map = Vec::new();

        for i in 0..self.size {
            for j in 0..self.size {
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

        let mut mesh = window.add_trimesh(mesh::points_to_mesh(self.height_map.clone(), self.size, self.scale), Vector3::new(1.0, 1.0, 1.0));
        mesh.set_color(self.color.0, self.color.1, self.color.2);
        
        let corrected = ((self.size - 1) as f32*self.scale)/2.0;
        let center = Point3::new(corrected, 0.0, corrected);
        let eye = Point3::new(corrected, 1.5*corrected, -2.0*corrected);
        
        let rot = 0.008; // Smooth rotation effect
        let mut arc_ball = ArcBall::new(eye, center); // Arc Ball Camera
        window.set_light(Light::StickToCamera);
        
        while !window.should_close() {
            arc_ball.set_yaw(arc_ball.yaw() + rot);
            window.render_with_camera(&mut arc_ball);
        }
    }
}
