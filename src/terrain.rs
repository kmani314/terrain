extern crate kiss3d;
extern crate nalgebra as na;

use na::Point2;
use na::Point3;

use kiss3d::window::Window;
use kiss3d::light::Light;

pub struct Terrain {
    height_map: Vec<Point3<f32>>,
    origin: Point2<u32>,
    x_size: usize,
    y_size: usize,
}

impl Terrain {
    pub fn new(origin: Point2<u32>, x_size: usize, y_size: usize) -> Terrain {
        Terrain {
            height_map: Vec::new(), 
            origin: origin,
            x_size: x_size,
            y_size: y_size,
        }
    }

    pub fn randomize(&self) {

    }

    pub fn render(&self) {
        let mut window = Window::new("terrain");
        let mut quad = window.add_quad(5f32, 5f32, self.x_size, self.y_size);

        window.set_light(Light::StickToCamera);

        quad.set_color(1f32, 1f32, 1f32); 
        while window.render() {}
    }
}
