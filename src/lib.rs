//! # fabmod: The F*cking Awesome Builder
//!
//! Programmatic 3D Graphics and CAD for Humans.
//!
pub mod shapes;
pub mod design;

// Need monstertruck publicly imported bc user needs it for fab macro_rules!
#[doc(hidden)]
pub use monstertruck;

