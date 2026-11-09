use anyhow::{Context,Result as AResult};
use cgmath::Vector3;
use monstertruck::modeling::bounding as mbounding;
use fab::shapes::*;

#[test]
fn set_concentric_cuboids() -> AResult<()> {

    let mut cube1 = cube(1.0);
    let mut cube2 = cube(1.0);

    cube2.translate(2.0 * Vector3::unit_x())?;
    cube1.set_concentric(&cube2)?;

    let center1 =
        mbounding::certified_solid_bounding_box(cube2.model())
            .with_context(||
                "Failed to extract bounding box of shape to be centered on.")?
            .center();

    let center2 =
        mbounding::certified_solid_bounding_box(cube2.model())
            .with_context(||
                "Failed to extract bounding box of shape to be centered on.")?
            .center();

    assert_eq!(center1, center2);

    Ok(())
}
