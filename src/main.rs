extern crate nalgebra as na;

mod terrain;
mod mesh;
mod shader;
mod noise;

use terrain::Terrain;
use terrain::Color;

fn main() {
    let mut terrain = Terrain::new(Color(0.439, 0.329, 0.243), 512, 0.015);
    terrain.generate_terrain();
    terrain.render();
}
