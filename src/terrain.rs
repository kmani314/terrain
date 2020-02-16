extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

use std::rc::Rc;
use std::cell::RefCell;

use kiss3d::{window::Window, light::Light, camera::ArcBall, resource::Material};
use na::{Point3, Vector3};
use crate::{mesh, noise, material};
use material::Color;

pub struct Terrain {
    height_map: Vec<f32>, // Abstract the height map away
    size: u32,
    amplitude: f32,
    background: material::Color,
}

impl Terrain {
    pub fn new(background: material::Color, size: u32) -> Terrain {
        Terrain {
            height_map: Vec::new(),
            size,
            amplitude: 1.0,
            background,
        }
    }

    pub fn generate_terrain(&mut self, persistence: f64, layers: u32, lacunarity: f64, scale: f64, amplitude: f32) {
        self.amplitude = amplitude;
        self.height_map = noise::get_noisy_map(self.size, persistence, layers, lacunarity, scale);
    }
    
    pub fn render(&mut self) {
        let mut window = Window::new("terrain");
        window.set_background_color(self.background.0, self.background.1, self.background.2);

        let mut mesh = window.add_trimesh(mesh::points_to_mesh(&self.height_map, self.size, self.amplitude), Vector3::new(1.0, 1.0, 1.0));

        mesh.recompute_normals();        
        // Need custom material for colors
        static THRESHOLDS: [f32; 3] = [0.3, 0.5, 0.8]; // This is stupid
        static COLORS: [Color; 4] = [Color(0.0, 0.0, 1.0), Color(1.0, 1.0, 0.0), Color(0.5, 0.5, 0.5), Color(1.0, 1.0, 1.0)];
        let material = Rc::new(RefCell::new(Box::new(material::TerrainMaterial::new(&THRESHOLDS, &COLORS)) as Box<dyn Material>));
        mesh.set_material(material);

        let corrected = ((self.size - 1) as f32)/2.0;
        let center = Point3::new(corrected, 0.0, corrected);
        let eye = Point3::new(corrected, 1.5*corrected, -2.0*corrected); // General zoomed out view
        
        let rot = 0.00; // Smooth rotation effect
        let mut arc_ball = ArcBall::new(eye, center); // Arc Ball Camera
        window.set_light(Light::StickToCamera);
        
        while !window.should_close() {
            arc_ball.set_yaw(arc_ball.yaw() + rot);
            window.render_with_camera(&mut arc_ball);
        }
    }
}
