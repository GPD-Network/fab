use anyhow::Result as AResult;
use cgmath::Vector3;

use fab::shapes::{cuboid, Cuboid, Shape3D};

fn main() -> AResult<()> {
    test_design_rs_api()
}

#[test]
fn test_main() -> AResult<()> {
    test_design_rs_api()
}


fn test_design_rs_api() -> AResult<()> {

    let punch = square_punch_3d(1.0, 0.2);

    Ok(())
}

fn unit_x() -> Vector3<f64> {
    Vector3::new(1.0, 0.0, 0.0)
}

fn unit_y() -> Vector3<f64> {
    Vector3::new(0.0, 1.0, 0.0)
}

fn unit_z() -> Vector3<f64> {
    Vector3::new(0.0, 0.0, 1.0)
}

fn square_punch_3d(sidelength: f64, thickness: f64) ->
    AResult<fab::shapes::CompositeShape3D> {

    let p1: Cuboid = cuboid(thickness, thickness, sidelength)?
        .translate(0.5 * (sidelength - thickness) * unit_x())?;

    let p2: Cuboid = cuboid(thickness, thickness, sidelength)?
        .translate(0.5 * (sidelength - thickness) * unit_y())?;

    let p3: Cuboid = cuboid(thickness, thickness, sidelength)?
        .translate(0.5 * (sidelength - thickness) * unit_z())?;


    p1.add(&p2)?.add(&p3)
}
