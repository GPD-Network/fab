use monstertruck::modeling::*;
use monstertruck::solid;
use monstertruck::step::save::{
    CompleteStepDisplay, StepHeaderDescriptor, StepModel
};

const PI: Rad<f64> = Rad(std::f64::consts::PI);


fn cylinder(base_origin: Point3, radius: f16, height: f16) ->
  anyhow::Result<Solid> {


    todo!()
}


fn main() -> anyhow::Result<()> {

    println!("Hello, world!\nThis is a test of MONSTERTRUCK!");

    // Unit cuboid centred on the origin of xy plane, sitting at
    // 0 elevation, one unit high.
    let cube: Solid = primitive::cuboid(BoundingBox::from_iter([
        Point3::new(-0.5, -0.5, 0.0),
        Point3::new(0.5, 0.5, 1.0),
    ]));

    // Cylinder of radius 0.3, axis along +Z, that pierces the cube top to bottom.
    let seed_z = builder::vertex(Point3::new(0.475, 0.0, -0.1));
    let rim = builder::revolve(
        &seed_z,
        Point3::new(0.0, 0.0, -0.1),
        Vector3::unit_z(),  // Tangent unit vector
        builder::SweepAngle::Closed,
        4,
    );
    let base = builder::try_attach_plane(&[rim])?;
    let mut cylinder: Solid = builder::extrude(&base, Vector3::unit_z() * 1.2);

    // Invert the cylinder so `and` performs a subtraction.
    cylinder.not();
    let anti_cylinder = cylinder;
    let punched = solid::and(&cube, &anti_cylinder, 0.05)?;

    // NOW DO ONE FOR EACH FACE
    // Cylinder of radius 0.3, axis along +Z, that pierces the cube top to bottom.
    let seed_ = builder::vertex(Point3::new(0.475, 0.0, -0.1));
    let rim = builder::revolve(
        &seed,
        Point3::new(0.0, 0.0, -0.1),
        Vector3::unit_z(),  // Tangent unit vector
        builder::SweepAngle::Closed,
        4,
    );
    let base = builder::try_attach_plane(&[rim])?;
    let mut cylinder: Solid = builder::extrude(&base, Vector3::unit_z() * 1.2);

    // Invert the cylinder so `and` performs a subtraction.
    cylinder.not();
    let anti_cylinder = cylinder;
    let punched = solid::and(&cube, &anti_cylinder, 0.05)?;

    //-- Now we create a neg-ball and use to smooth corners.
    //
    // Ball shares center vertex with that of the cube
    let ball_center_point = Point3::new(0.0, 0.0, 0.5);
    // Create a half-circle 1D "wire"
    let half_circle: Wire = builder::revolve(
      &builder::vertex(ball_center_point),
      Point3::new(0.0, 1.0, 0.0),
      Vector3::unit_z(),
      builder::SweepAngle::Partial(PI),
      10);
    // Create a 2D spherical shell by revolving the shape with wire halfcircle
    let spherical_shell: Shell  = builder::revolve_wire(
      &half_circle,
      ball_center_point,
      Vector3::unit_x(),
      builder::SweepAngle::Closed,
      10
    );
    // Create the ball and anti-ball from that
    let mut ball = Solid::new(vec![spherical_shell]);
    ball.not();
    let antiball = ball;

    // Rounded corners via intersection of antiball and existing punched cube

    // Flatten the topology and render as STEP.
    let compressed = punched.compress();

    // Create STEP-formatted representation of the piece.
    let step = CompleteStepDisplay::new(
        StepModel::from(&compressed),
        StepHeaderDescriptor {
            organization_system: "monstertruck-doc".to_owned(),
            ..Default::default()
        },
    )
    .to_string();

    assert!(step.starts_with("ISO-10303-21;"));
    assert!(step.contains("MANIFOLD_SOLID_BREP"));

    std::fs::write("punch-cube.step", step)?;

    Ok(())
}
