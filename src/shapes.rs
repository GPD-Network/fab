use std::f64::consts::TAU;

use anyhow::Context;
use anyhow::Result as AResult;
use cgmath::{Point3, Vector3};
use monstertruck::modeling::*;


fn normal(azimuth_rad: f64, inclination_rad: f64) -> AResult<Vector3<f64>> {

    Ok(
        Vector3::new(
            inclination_rad.sin() * azimuth_rad.cos(),
            inclination_rad.sin() * azimuth_rad.sin(),
            inclination_rad.cos(),
        )
    )
}


pub fn circle_wire(
    base_origin: Point3<f64>,
    radius: f64,
    azimuth_turns: f64,
    inclination_turns: f64
) -> AResult<Wire> {

    // Replace params in turns with polar coordinate variables in radians
    let theta_radians = azimuth_turns * TAU;
    let phi_radians = inclination_turns * TAU;

    let circle_normal = normal(theta_radians, phi_radians)?;

    Ok(
        builder::revolve(
            &builder::vertex(base_origin),
            Point3::new(radius, 0.0, 0.0),
            circle_normal,
            builder::SweepAngle::Closed,
            10
        )
    )
}


pub fn circle_face(
        base_origin: Point3<f64>,
        radius: f64,
        azimuth_turns: f64,
        inclination_turns: f64
)  -> AResult<Face> {

    Ok(
        builder::try_attach_plane(&[
            circle_wire(base_origin, radius, azimuth_turns, inclination_turns)?
        ])?
    )
}


pub fn cylinder(
    base_origin: Point3<f64>,
    radius: f64,
    height: f64,
    // final two angles are given in turns, ie multiples of \tau
    axis_azimuth: f64,
    axis_inclination: f64) -> AResult<Solid> {

    // Given the aziumuth and inclination
    let cylinder_axis_normal = normal(axis_azimuth, axis_inclination)?;

    Ok(
        builder::extrude(

            &circle_face(
                base_origin,
                radius,
                axis_azimuth,
                axis_inclination,
            )?,

            height * cylinder_axis_normal
        )
    )
}


/// Rotate a solid around its calculated center, if
pub fn rotate_xyz(
    solid: Solid,
    x_turns: f64,
    y_turns: f64,
    z_turns: f64,
) -> AResult<Solid> {

    let center: Point3<f64> =
        bounding::certified_solid_bounding_box(&solid)
            .with_context(||  "Failed to extract bounding box.")?
            .center();

    let solid = builder::rotated(
        &solid,
        center,
        Vector3::unit_x(),
        Rad(x_turns * TAU),
    );

    let solid = builder::rotated(
        &solid,
        center,
        Vector3::unit_y(),
        Rad(y_turns * TAU),
    );

    let solid = builder::rotated(
        &solid,
        center,
        Vector3::unit_z(),
        Rad(z_turns * TAU),
    );

    Ok(solid)
}


pub fn cuboid(xlen: f64, ylen: f64, zlen: f64)
    -> AResult<Solid> {

    Ok(
        primitive::cuboid(
            BoundingBox::from_iter([
                Point3::origin(),
                Point3::new(xlen, ylen, zlen)
            ])
        )
    )
}


pub fn cube(sidelen: f64) -> AResult<Solid> {
    Ok(cuboid(sidelen, sidelen, sidelen)?)
}


