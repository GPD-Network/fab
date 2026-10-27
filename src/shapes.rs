//! # Create, manipulate, and inspect basic shapes
//!
//! This module provides convenience wrappers around more primitive features of
//! [monstertruck](https://docs.rs/monstertruck/latest/monstertruck/).
//!
//! # Examples
//!
//! Cuboid is a generalized cube, aka rectangular prism, can use either name:
//! [cuboid] or [rectangular_prism].
//!
//! ```
//! use fab::shapes::{cuboid,cube}
//!
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
    meshing::{
        analyzers::CalcVolume,
        tessellation::{MeshableShape,MeshedShape}
    },
    modeling::{
        bounding as mbounding,
        builder as mbuilder,
        primitive as mprimitive,
        BoundingBox as MBoundingBox,
        Face as MFace,
        Solid as MSolid,
        Wire as MWire,
    },
    solid as msolid
};

pub trait HasVolume {
    /// We define volume for implementations of Shape3D trait
    fn volume(&self) -> f64;
}

pub trait Shape3D: HasVolume {
    /// The presence of a monstertruck::builder::Solid
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

#[derive(Debug,PartialEq)]
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

// Private helper to create the monstertruck Solid model of a cuboid
fn cuboid_model(xlen: f64, ylen: f64, zlen: f64) -> AResult<MSolid> {
    Ok(
        mprimitive::cuboid(
            MBoundingBox::from_iter([
                Point3::origin(),
                Point3::new(xlen, ylen, zlen),
            ])
        )
    )
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
/// Use [`ball`], which wraps [`Ball::new`], to create one.
///
/// # Examples
/// ```
/// ```
#[derive(Debug,PartialEq)]
pub struct Ball {
    radius: f64,
    center: Point3<f64>,
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
        let model = ball_model(radius, center)?;

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
pub fn ball(radius: f64, center: Point3<f64>) -> AResult<Ball> {
    Ball::new(radius, center)
}

// Create intermediate monstertruck Solid model for use by Ball constructor
fn ball_model(radius:f64, center: Point3<f64>) -> AResult<MSolid> {
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
    radius: f64,
    azimuth_turns: f64,
    inclination_turns: f64,
    base_origin: Point3<f64>,
) -> AResult<MFace> {

    Ok(mbuilder::try_attach_plane(
        &[
            circle_wire(
                base_origin,
                radius,
                azimuth_turns,
                inclination_turns
            )?
        ])?
    )
}

/// A cylinder with properties radius, height, and axis pointing located at `base_origin`
#[derive(Debug,PartialEq)]
pub struct Cylinder {
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

    /// Create new Cylinder instance
    pub fn new(
        radius: f64,
        height: f64,
        axis_azimuth: f64,
        axis_inclination: f64,
    ) -> AResult<Cylinder> {

        let model = cylinder_model(
            radius,
            height,
            axis_azimuth,
            axis_inclination,
        )?;

        Ok(Cylinder {
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

/// Cylinder anchored with the base of its center at the Euclidian origin; its
/// axis is rotated according to the azimuth (rotation in xy plane) and
/// inclination (rotation "down" from aligned with the z axis, where 0 is
/// aligned with the axis).
pub fn cylinder(
    radius: f64,
    height: f64,
    axis_azimuth: f64,
    axis_inclination: f64,
) -> AResult<Cylinder> {
    Cylinder::new(
        radius,
        height,
        axis_azimuth,
        axis_inclination,
    )
}
// Wraps monstertruck primitives to create a monstertruck solid cylinder
fn cylinder_model(
    radius: f64,
    height: f64,
    axis_azimuth: f64,
    axis_inclination: f64,
) -> AResult<MSolid> {
    // Given the azimuth and inclination
    let cylinder_axis_normal = unit_direction(axis_azimuth, axis_inclination)?;

    Ok(
        mbuilder::extrude(
            &circle_face(
                radius,
                axis_azimuth,
                axis_inclination,
                Point3::origin(),
            )?,
            height * cylinder_axis_normal
        )
    )
}

/// Representation of 3D monstertruck solid made from composing other solids
pub struct CompositeShape3D {
    pub model: MSolid
}
impl_shape3d!(CompositeShape3D);

/// A composite shape has no volume formula; this uses monstertruck triangulation
impl HasVolume for CompositeShape3D {
    fn volume(&self) -> f64 {

        self.model()
            .triangulation(TRIANGULATION_TOL)
            .to_polygon()
            .volume()
    }
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

    // Replace params given in turns with polar coordinate variables in radians
    let theta_radians = azimuth_turns * TAU;
    let phi_radians = inclination_turns * TAU;

    // Circles' normal is ẑ if phi=0; theta is xy rotation, phi angle from ẑ
    let circle_normal = unit_direction(theta_radians, phi_radians)?;

    Ok(
        // Use monstertruck's builder to revolve a point at x=radius 2
        mbuilder::revolve(
            &mbuilder::vertex(base_origin),
            Point3::new(radius, 0.0, 0.0),
            circle_normal,
            mbuilder::SweepAngle::Closed,
            10,
        )
    )
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

/// Add two fab shapes together to get a third that joins the two, if possible
///
/// # Examples
///
/// ```
/// use fab::shapes::*;
///
/// // Cubes/cuboids have their origin corner at Euclidian origin
/// let bigger_cube = cube(1.0);
/// let smaller_cube = cube(0.5);
/// let result_cube = bigger_cube.subtract(smaller_cube);
/// assert_eq!(result_cube.volume(), 1.0 - 0.125);
/// ```
pub fn add(shape1: &impl Shape3D, shape2: &impl Shape3D) -> AResult<CompositeShape3D> {

    let result_model = msolid::or(
        shape1.model(),
        shape2.model(),
        TRIANGULATION_TOL
    )?;

    Ok(
        CompositeShape3D { model: result_model }
    )
}

/// Subtract one fab shape (shape2) from another (shape1) to get a third
/// resulting shape represented as a [CompositeShape3D].
///
pub fn subtract(
    shape1: &impl Shape3D,
    shape2: &impl Shape3D
) -> AResult<CompositeShape3D> {

    // Clone shape 2 model because we'll need to negate it to prep for subtract
    let mut to_remove = shape2.model().clone();

    // Prepare shape2 to subtract from shape 1
    to_remove.not();

    // Create resulting model that takes shape 2 from shape 1
    let result_model = msolid::and(
        shape1.model(),
        &to_remove,
        TRIANGULATION_TOL
    )?;

    Ok (
        CompositeShape3D { model: result_model }
    )
}

/// Tolerance used by monstertruck in creating printable 3D models
const TRIANGULATION_TOL: f64 = 0.01;

