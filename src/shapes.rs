
use monstertruck::modeling::*;

use cgmath::Rad;

pub fn cylinder(
    base_origin: Point3,
    radius: f32,
    height: f32) -> anyhow::Result<Solid> {



    todo!()
}


pub fn circle_wire(
    center: Point3,
    radius: f32,
    normal: Vector3,

) -> anyhow::Result<Wire> {

    // Given normal vector, calculate a point on the oriented circle


    Ok(
        builder::revolve(
            &builder::vertex(center),

        )
    )
}


pub fn circle2d(
    center: Point3,
    radius: f32,
    normal: Vector3)  -> anyhow::Result<Face> {

    todo!()
}
