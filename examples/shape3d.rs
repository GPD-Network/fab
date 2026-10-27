
use anyhow::Result as AResult;
use monstertruck::modeling::Solid as MSolid;

use fab::shapes::{CompositeShape3D, HasVolume, Shape3D};


fn main() -> AResult<()> {

    // Create a house with a square base, length one, and an regular triangular
    // prism for a roof.
    let wall_length = 1.0;
    let one_house = SimpleHouse { wall_length };
    assert_eq!(one_house.volume(), 1.5);

    Ok(())
}

struct SimpleHouse {
    wall_length: f64
}
impl_shape

impl Shape3D for SimpleHouse {
    fn model(&self) -> AResult<MSolid> {
        simple_house_model()
    }
}

fn simple_house_model() -> AResult<CompositeShape3D> {}

impl HasVolume for SimpleHouse {
    fn volume(&self) -> f64 {

        todo!()
    }
}

// Define a new shape, the triangular prism, the kind that make rainbows.
// Make it equilateral and regular, meaning sides are all the same length.
// Set the height beyond this.
struct TriangularPrism {
    side_length: f64,
    height: f64

}

