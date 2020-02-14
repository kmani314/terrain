extern crate nalgebra as na;

mod terrain;
mod mesh;
mod shader;
mod noise;

use terrain::Terrain;
use terrain::Color;

fn main() {
    let mut terrain = Terrain::new(Color(0.439, 0.329, 0.243), 32, 1.0);
    terrain.random();
    terrain.render();
}
