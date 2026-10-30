use anyhow::Result as AResult;
use cgmath::Vector3;

use fab::shapes::{cube,HasVolume,Shape3D};


fn main() -> AResult<()> {
    _test_cuboid_volumes()?;
    _test_cuboid_arithmetic()
}

#[test]
fn test_cuboid_volumes() -> AResult<()> {
    _test_cuboid_volumes()
}

#[test]
fn test_cuboid_arithmetic() -> AResult<()> {
    _test_cuboid_arithmetic()
}


fn _test_cuboid_volumes() -> AResult<()> {
    // Cubes/cuboids have their origin corner at Euclidian origin
    let bigger_cube = cube(1.0)?;
    assert_eq!(bigger_cube.volume(), 1.0);

    // Check that a smaller cube has expected volume; mutable bc we'll move it
    let smaller_cube = cube(0.5)?;
    assert_eq!(smaller_cube.volume(), 0.125);

    Ok(())
}


fn _test_cuboid_arithmetic() -> AResult<()> {


    // Subtract the smaller cube from the larger, is volume reduced as expected?
    let bigger_cube = cube(1.0)?;
    // In this case we create a larger cube than needed,...
    let mut smaller_cube = cube(0.6)?;
    // ...and translate it an amount that offsets the oversize so have overlap
    smaller_cube.translate(-1.0 * Vector3::new(0.1, 0.1, 0.1))?;
    // All set to now subtract smaller from larger
    let result_cube = bigger_cube.subtract(&smaller_cube)?;
    assert_eq!(result_cube.volume(), 1.0 - 0.125);

    // Now test addition, starting with a bigger cube
    let bigger_cube = cube(1.0)?;
    // Create a smaller cube of volume 0.125
    let mut smaller_cube = cube(0.5)?;
    // Translate, half of it in bigger
    smaller_cube.translate(Vector3::new(0.25, 0.25, 0.75))?;
    // Ready to add the two
    let joined = bigger_cube.add(&smaller_cube)?;

    // Check volume = vol. of A + volume of B − volume of their intersection
    // --> Here the overlap is 0.25 × 0.25 × 0.25 = 0.0625
    assert_eq!(joined.volume(), 1.0 + 0.125 - 0.0625);

    Ok(())
}

