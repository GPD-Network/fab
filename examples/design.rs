use std::f64::consts::PI;

use anyhow::Result as AResult;

use cgmath::Vector3;

use fab::shapes::{cube, cuboid, Cuboid, Shape3D, HasVolume};

fn main() -> AResult<()> {
    test_design_rs_api()
}

#[test]
fn test_main() -> AResult<()> {
    test_design_rs_api()
}


fn test_design_rs_api() -> AResult<()> {

    let outer_cube = cube(1.0);
    let punched_cube = punch(outer_cube);


    let punch_length: f64 = 1.2;
    let punch_thickness: f64 = 0.2;
    // let punch = square_punch_3d(punch_length, 0.2)?;

    assert_eq!(outer_cube.volume(), 1.0);


    // Now create the punched outer cube and check the volumes
    let res = outer_cube.subtract(&punch)?;

    assert_eq!(punch.volume(), expected_volume);
    assert_eq!(res.volume(), outer_cube.volume() - punch.volume());

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

fn square_punch_3d(cube: Cube, sidelength: f64, thickness: f64) ->
    AResult<fab::shapes::CompositeShape3D> {

    let p1: Cuboid = cuboid(thickness, thickness, sidelength)?;

    let mut p2: Cuboid = cuboid(thickness, thickness, sidelength)?;
    p2.translate(0.5 * (sidelength - thickness) * unit_y())?;
    p2.rotate_xyz(0.0, PI / 2.0, 0.0)?;
    p2.rotate_xyz(0.0, 0.0, PI / 2.0)?;

    let mut p3: Cuboid = cuboid(thickness, thickness, sidelength)?;
    p3.translate(0.5 * (sidelength - thickness) * unit_z())?;
    p3.rotate_xyz(0.0, PI / 2.0, 0.0)?;

    p2.set_concentric(&p1)?;
    p3.set_concentric(&p1)?;

    p1.add(&p2)?.add(&p3)
}
