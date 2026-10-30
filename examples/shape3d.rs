use std::f64::consts::{PI};

use anyhow::Result as AResult;
use cgmath::{Point3,Vector3};
use monstertruck::modeling::{
    builder as mbuilder,
    Solid as MSolid,
    Surface as MSurface,
    Wire as MWire
};

use fab::{
    impl_shape3d,
    shapes::{add,CompositeShape3D,HasVolume,Shape3D,subtract}
};


fn main() -> AResult<()> {
    test_triangular_prism_volume()
}

#[test]
fn main_test() -> AResult<()> {
    test_triangular_prism_volume()
}

fn test_triangular_prism_volume() -> AResult<()> {

    let triangle_side_length = 1.0;
    let prism_length = 1.0;

    // Initialize a new (regular) tri. prism with given dimensions
    let tp = TriangularPrism::new(triangle_side_length, prism_length)?;

    // Make sure volume is calculated sensibly and correctly
    assert_eq!(
        tp.volume(),
        0.5 * triangle_side_length.powi(2) * (PI / 3.0).sin() * prism_length
    );

    Ok(())
}

// Define a new shape, the triangular prism, the kind that make rainbows.
// Make it equilateral and regular, meaning sides are all the same length.
// Set the height beyond this.
#[derive(Debug,PartialEq)]
struct TriangularPrism {
    triangle_side_length: f64,
    triangle_height: f64,
    prism_length: f64,
    model: MSolid
}

impl TriangularPrism {

    fn new(triangle_side_length: f64, prism_length: f64) -> AResult<TriangularPrism> {

        let triangle_height = (PI / 3.0).sin() * triangle_side_length;

        let model = triangular_prism_model(triangle_side_length, prism_length)?;

        Ok(
            TriangularPrism {
                triangle_side_length,
                triangle_height,
                prism_length,
                model
            }
        )
    }
}

fn triangular_prism_model(
    triangle_side_length: f64,
    prism_length: f64
) -> AResult<MSolid> {

    // Use trigonometry to find height of equilateral triangle
    // sin(theta) = opposite / hypotenuse -> opposite = sin(theta) * hypotenuse
    // theta = 60 degrees = PI / 3.0
    let tri_height = (PI / 3.0).sin() * triangle_side_length;

    // Create vertices for our equilateral triangle base in xy plane
    let v0 = mbuilder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v1 = mbuilder::vertex(Point3::new(triangle_side_length, 0.0, 0.0));
    let v2 = mbuilder::vertex(Point3::new(
        triangle_side_length / 2.0,
        tri_height,
        0.0,
    ));

    // First we build a wire, line segments that connects the three vertices
    let wire: MWire = vec![
        mbuilder::line(&v0, &v1),
        mbuilder::line(&v1, &v2),
        mbuilder::line(&v2, &v0),
    ]
    .into();

    // mtruck's try_attach_plane expects vector of Wire instances
    let wires = vec![wire];

    // Expect success in attaching plane
    let triangle_plane_result = mbuilder::try_attach_plane::<_, MSurface>(wires);
    let is_ok = triangle_plane_result.is_ok();
    assert!(is_ok);

    // Must extract the value of the result to use it
    let triangle_plane = triangle_plane_result?;

    // Extrude the triangle in xy plane along z axis; it has triangle base
    let triangular_prism: MSolid = mbuilder::extrude(
        &triangle_plane,
        prism_length * Vector3::unit_z()
    );

    // triangular prism should have five faces
    assert_eq!(triangular_prism.boundaries()[0].len(), 5);

    Ok(triangular_prism)
}

impl HasVolume for TriangularPrism {
    fn volume(&self) -> f64 {
        0.5 * self.triangle_side_length
            * self.triangle_height
            * self.prism_length
    }
}
impl_shape3d!(TriangularPrism);
