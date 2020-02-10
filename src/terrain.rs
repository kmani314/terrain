extern crate kiss3d;
extern crate nalgebra as na;

use na::Point2;

use kiss3d::window::Window;
use kiss3d::light::Light;

pub struct Terrain {
    height_map: Vec<Vec<f32>>,
    origin: Point2<u32>,
    x_size: usize,
    y_size: usize,
}

impl Terrain {
    pub fn new(origin: Point2<u32>, x_size: usize, y_size: usize) -> Terrain {
        Terrain {
            height_map: vec![vec![0f32; x_size]; y_size],
            origin: origin,
            x_size: x_size,
            y_size: y_size,
        }
    }

    pub fn randomize(&self) {

    }

    pub fn render(&self) {
        let mut window = Window::new("terrain");
        while window.render() {}
    }
}
