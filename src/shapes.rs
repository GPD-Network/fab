//! # Create, manipulate, and inspect basic shapes
//!
//! This module provides convenience wrappers around more primitive features of
//! [monstertruck](https://docs.rs/monstertruck/latest/monstertruck/).
//!
//! # Examples
//!
//! Cuboid is a generalized cube, also called a rectangular prism, use either name
//!
//! ```
//! use fab::shapes::{cuboid,rectangular_prism,cube}
//! ```
//!
//! Can also make cylinders
use std::f64::consts::{PI, TAU};

use anyhow::Context;
use anyhow::Result as AResult;
use cgmath::{Point3, Rad, Vector3};
// The following monstertruck imports are perhaps overkill,
// but I was getting confused about what came from this
// library, fab, and what came from monstertruck.
use monstertruck::{
    core::control_point::ControlPoint,
    modeling::{
        bounding as mbounding,
        builder as mbuilder,
        primitive as mprimitive,
        BoundingBox as MBoundingBox,
        Face as MFace,
        Solid as MSolid,
        Wire as MWire,
    },
};

pub trait HasVolume {
    fn volume(&self) -> f64;
}

pub trait Shape3D: HasVolume {
    fn model(&self) -> &MSolid;
}

macro_rules! impl_shape3d {
    ($shape:ty) => {
        impl Shape3D for $shape {
            fn model(&self) -> &MSolid {
                &self.model
            }
        }
    };
}

macro_rules! f64_accessors {
    ($shape:ty { $($field:ident),* $(,)? }) => {
        impl $shape {
            $(
                pub fn $field(&self) -> f64 {
                    self.$field
                }
            )*
        }
    };
}

pub struct Cuboid {
    xlen: f64,
    ylen: f64,
    zlen: f64,
    pub model: MSolid,
}

f64_accessors!(Cuboid { xlen, ylen, zlen });

impl Cuboid {
    pub fn new(xlen: f64, ylen: f64, zlen: f64) -> AResult<Cuboid> {
        let model = cuboid_model(xlen, ylen, zlen)?;

        Ok(Cuboid {
            xlen,
            ylen,
            zlen,
            model,
        })
    }
}

impl HasVolume for Cuboid {
    fn volume(&self) -> f64 {
        self.xlen() * self.ylen() * self.zlen()
    }
}

impl_shape3d!(Cuboid);

/// Create a [hexahedron](https://en.wikipedia.org/wiki/Hexahedron) with rectangular sides, i.e., a generalized cube
pub fn cuboid(xlen: f64, ylen: f64, zlen: f64) -> AResult<Cuboid> {
    Cuboid::new(xlen, ylen, zlen)
}

// Private helper to create the monstertruck Solid model of
fn cuboid_model(xlen: f64, ylen: f64, zlen: f64) -> AResult<MSolid> {
    Ok(mprimitive::cuboid(MBoundingBox::from_iter([
        Point3::origin(),
        Point3::new(xlen, ylen, zlen),
    ])))
}

pub struct Cube {
    sidelen: f64,
    model: MSolid,
}

f64_accessors!(Cube { sidelen });

impl Cube {
    pub fn new(sidelen: f64) -> AResult<Cube> {
        let model = cube_model(sidelen)?;

        Ok(Cube { sidelen, model })
    }
}

impl HasVolume for Cube {
    fn volume(&self) -> f64 {
        self.sidelen().powi(3)
    }
}

impl_shape3d!(Cube);

pub fn cube(sidelen: f64) -> AResult<Cube> {
    Cube::new(sidelen)
}

/// A regular [hexahedron](https://en.wikipedia.org/wiki/Hexahedron)
fn cube_model(sidelen: f64) -> AResult<MSolid> {
    cuboid_model(sidelen, sidelen, sidelen)
}

/// A 3D ball represented by its center, radius, and `monstertruck` solid model.
/// See [`monstertruck`'s `Solid` struct](https://docs.rs/monstertruck-topology/0.4.1/monstertruck_topology/struct.Solid.html)
///
/// Use [`ball`], which wraps [`Ball::new`] to create one.
///
/// # Examples
/// ```
pub struct Ball {
    center: Point3<f64>,
    radius: f64,
    pub model: MSolid,
}

f64_accessors!(Ball { radius });

impl HasVolume for Ball {
    /// Use exact calculation for volume of a ball
    fn volume(&self) -> f64 {
        (4.0 / 3.0) * PI * self.radius().powi(3)
    }
}

impl_shape3d!(Ball);

impl Ball {
    /// Create new ball with given radius and centre point
    pub fn new(radius: f64, center: Point3<f64>) -> AResult<Self> {
        let model = ball_model(center, radius)?;

        Ok(Self {
            radius,
            center,
            model,
        })
    }

    /// Returns ball center
    pub fn center(&self) -> Point3<f64> {
        self.center
    }
}

/// Convenience constructor for a new Ball instance
pub fn ball(center: Point3<f64>, radius: f64) -> AResult<Ball> {
    Ball::new(radius, center)
}

// Create intermediate monstertruck Solid model for use by Ball constructor
fn ball_model(center: Point3<f64>, radius: f64) -> AResult<MSolid> {
    Ok(MSolid::new(vec![mbuilder::revolve_wire(
        &circle_wire(center, radius, 0.0, 0.0)?,
        center,
        Vector3::unit_y(),
        mbuilder::SweepAngle::Closed,
        10,
    )]))
}

/// A circular face, i.e., a 2D circular plane
pub fn circle_face(
    base_origin: Point3<f64>,
    radius: f64,
    azimuth_turns: f64,
    inclination_turns: f64,
) -> AResult<MFace> {
    Ok(mbuilder::try_attach_plane(&[circle_wire(
        base_origin,
        radius,
        azimuth_turns,
        inclination_turns,
    )?])?)
}

pub struct Cylinder {
    base_origin: Point3<f64>,
    radius: f64,
    height: f64,
    axis_azimuth: f64,
    axis_inclination: f64,
    model: MSolid,
}

f64_accessors!(Cylinder {
    radius,
    height,
    axis_azimuth,
    axis_inclination,
});

impl Cylinder {
    pub fn base_origin(&self) -> Point3<f64> {
        self.base_origin
    }

    pub fn new(
        base_origin: Point3<f64>,
        radius: f64,
        height: f64,
        axis_azimuth: f64,
        axis_inclination: f64,
    ) -> AResult<Cylinder> {
        let model = cylinder_model(
            base_origin,
            radius,
            height,
            axis_azimuth,
            axis_inclination,
        )?;

        Ok(Cylinder {
            base_origin,
            radius,
            height,
            axis_azimuth,
            axis_inclination,
            model,
        })
    }
}

impl HasVolume for Cylinder {
    fn volume(&self) -> f64 {
        PI * self.radius().powi(2) * self.height()
    }
}

impl_shape3d!(Cylinder);

/// Cylinder anchored at its base's center; points along axis angle parameters
/// given as fractions [0.0, 1.0] of a full turn around 2π radians.
pub fn cylinder(
    base_origin: Point3<f64>,
    radius: f64,
    height: f64,
    axis_azimuth: f64,
    axis_inclination: f64,
) -> AResult<Cylinder> {
    Cylinder::new(
        base_origin,
        radius,
        height,
        axis_azimuth,
        axis_inclination,
    )
}

fn cylinder_model(
    base_origin: Point3<f64>,
    radius: f64,
    height: f64,
    axis_azimuth: f64,
    axis_inclination: f64,
) -> AResult<MSolid> {
    // Given the azimuth and inclination
    let cylinder_axis_normal = unit_direction(axis_azimuth, axis_inclination)?;

    Ok(mbuilder::extrude(
        &circle_face(
            base_origin,
            radius,
            axis_azimuth,
            axis_inclination,
        )?,
        height * cylinder_axis_normal,
    ))
}

/// Find the normal to the plane defined by the azimuthal and inclination angles
pub fn unit_direction(azimuth_rad: f64, inclination_rad: f64) -> AResult<Vector3<f64>> {
    Ok(Vector3::new(
        inclination_rad.sin() * azimuth_rad.cos(),
        inclination_rad.sin() * azimuth_rad.sin(),
        inclination_rad.cos(),
    ))
}

/// Create a 1D circle
pub fn circle_wire(
    base_origin: Point3<f64>,
    radius: f64,
    azimuth_turns: f64,
    inclination_turns: f64,
) -> AResult<MWire> {
    // Replace params in turns with polar coordinate variables in radians
    let theta_radians = azimuth_turns * TAU;
    let phi_radians = inclination_turns * TAU;

    let circle_normal = unit_direction(theta_radians, phi_radians)?;

    Ok(mbuilder::revolve(
        &mbuilder::vertex(base_origin),
        Point3::new(radius, 0.0, 0.0),
        circle_normal,
        mbuilder::SweepAngle::Closed,
        10,
    ))
}

/// Rotate a solid around its auto-calculated center
pub fn rotate_xyz(
    solid: MSolid,
    x_turns: f64,
    y_turns: f64,
    z_turns: f64,
) -> AResult<MSolid> {
    let center: Point3<f64> = mbounding::certified_solid_bounding_box(&solid)
        .with_context(|| "Failed to extract bounding box.")?
        .center();

    let solid = mbuilder::rotated(
        &solid,
        center,
        Vector3::unit_x(),
        Rad(x_turns * TAU),
    );

    let solid = mbuilder::rotated(
        &solid,
        center,
        Vector3::unit_y(),
        Rad(y_turns * TAU),
    );

    let solid = mbuilder::rotated(
        &solid,
        center,
        Vector3::unit_z(),
        Rad(z_turns * TAU),
    );

    Ok(solid)
}
