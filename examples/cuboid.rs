use anyhow::Result as AResult;

use fab::shapes::{cube, cuboid, HasVolume};

fn main() -> AResult<()> {
    test_cuboid_volumes()
}

#[test]
fn test_main() -> AResult<()> {
    test_cuboid_volumes()
}


fn test_cuboid_volumes() -> AResult<()> {

    // pron: cue-boyd-ee
    let cuboidey = cuboid(1.0 , 2.0 , 4.0)?;

    // pron: cue-bee
    let cubey = cube(2.0)?;

    assert_eq!(cuboidey.volume(), cubey.volume());

    Ok(())
}
