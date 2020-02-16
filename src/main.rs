extern crate nalgebra as na;

mod terrain;
mod mesh;
mod material;
mod noise;

use terrain::Terrain;
use material::Color;

fn main() {
    let mut terrain = Terrain::new(Color(0.529, 0.807, 0.922), 256);
    terrain.generate_terrain(0.3, 8, 2.4, 10.0, 10.0);
    terrain.render();
}
