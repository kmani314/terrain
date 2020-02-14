extern crate nalgebra as na;

mod terrain;
mod mesh;
mod shader;
mod noise;

use terrain::Terrain;
use terrain::Color;

fn main() {
    let mut terrain = Terrain::new(Color(0.439, 0.329, 0.243), 256);

    terrain.generate_terrain(0.3, 4, 3.0, 10.0, 10.0);
    terrain.render();
}
