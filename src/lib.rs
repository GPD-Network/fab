//! # fabmod: The F*cking Awesome Builder
//!
//! Programmatic 3D Graphics and CAD for Humans.
//!
//!
//!
//! ## Examples
//!
//! ```
//! use monstertruck::builder::{Point3,Solid};
//! use fab::shapes::{cuboid,cube,cylinder};
//!
//! // Create a regular cuboid, i.e., a cube
//! let cuboidie: Shape = cuboid(3.0, 3.0, 3.0);
//! // Create an equivalent cube directly
//! let cubie: Shape = cube(3.0);
//! // Test equivalence
//! assert_eq!(cuboidie, cubie);
//! // Test cuboidie not equal to another cuboid with different dimensions
//! let cuboider: Shape = cuboid(1.0, 2.0, 3.0);
//! assert_ne!(cuboidie, cuboider);
//!
//! //== Create a cylinder with same volume of cuboider
//! // Cylinder's base centered at x=1, y=z=0
//! let base_origin = Point3::new(1.0, 0.0, 0.0);
//! // Set a base radius of 1
//! let radius = 1.0;
//!
//! // Now to find height of cyllie to equal cuboider volume (1*2*3 = 6)
//! let target_volume = 6.0;
//! // We need pi because the volume of a cylinder is pi * r^2 * h
//! let pi_approx: f64 = 3.14159;
//! // Get cyllies height by scaling the target_volume by the circle's area
//! let cyllies_height = target_volume / (pi_approx * radius * radius);
//!
//! // For fun we make the cylinder's axis align along unit vector 45deg, 45deg
//! let fortyfive_deg_in_turns = 1.0 / 8.0;
//!
//! let cyllie = cylinder(
//!   base_origin,
//!   radius,
//!   cyllies_height,
//!   fortyfive_deg_in_turns,
//!   fortyfive_deg_in_turns
//! );
//! ```

pub mod shapes;
pub mod design;

// Need monstertruck publicly imported bc user needs it for fab macro_rules!
#[doc(hidden)]
pub use monstertruck;

