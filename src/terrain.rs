extern crate kiss3d;

use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::resource::Mesh;

use crate::mesh;

pub struct Color(pub f32, pub f32, pub f32);

pub struct Terrain {
    height_map: Mesh, // Abstract the height map away
    x_size: usize,
    y_size: usize,
    color: Color
}

impl Terrain {
    pub fn new(color: Color, x_size: usize, y_size: usize) -> Terrain {
        // TODO: height map stuff on new()
        /*Terrain {
            height_map: , 
            x_size: x_size,
            y_size: y_size,
            color: color,
        }*/
    }

    pub fn render(&self) {
        let mut window = Window::new("terrain");
        
        // TODO: mesh stuff
        window.set_light(Light::StickToCamera);

        while window.render() {}
    }
}
