use std::f64::consts::PI;
use fab::shapes::{ball,Ball,HasVolume};
use anyhow::Result as AResult;

use monstertruck::{
    core::control_point::ControlPoint,
    modeling::Point3,
};

fn main() -> AResult<()> {

    // Two equivalent ways to create a ball at the origin
    let radius = 2.0;
    let ball1 = ball(radius, Point3::origin())?;
    let ball2 = Ball::new(radius, Point3::origin())?;

    // Check that their dimensions are equal
    assert_eq!(ball1, ball2);

    // Check reported volumes equal the expected identity
    assert_eq!(ball1.volume(), (4.0 /3.0) * PI * radius.powi(3));
    assert_eq!(ball1.volume(), ball2.volume());

    Ok(())
}
