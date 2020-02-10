extern crate nalgebra as na;

mod terrain;

use na::Point2;
use terrain::Terrain;

fn main() {
    // generate terrain and initialize window and shaders
    let terrain = Terrain::new(Point2::new(0, 0), 256, 256);
    terrain.render();
}
