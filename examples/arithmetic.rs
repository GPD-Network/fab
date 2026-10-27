
use fab::shapes::*;

//
fn main() -> AResult<()> {
    // Cubes/cuboids have their origin corner at Euclidian origin
    let bigger_cube = cube(1.0);
    assert_eq!(result_cube.volume(), 1.0);
    let smaller_cube = cube(0.5);
    assert_eq!(result_cube.volume(), 0.125);
    let result_cube = bigger_cube.subtract(smaller_cube);
    assert_eq!(result_cube.volume(), 1.0 - 0.125);
}
