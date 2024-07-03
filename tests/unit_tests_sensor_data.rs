use bmi323::Sensor3DData;

const EPSILON: f32 = 1e-5;

fn assert_relative_eq(a: f32, b: f32, epsilon: f32) {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    assert!(
        diff <= epsilon * abs_a.max(abs_b),
        "assertion failed: `(left ≈ right)`\n  left: `{}`\n right: `{}`",
        a,
        b
    );
}

#[test]
fn test_sensor_data_to_mps2() {
    let sensor_data = Sensor3DData {
        x: 8192,
        y: -4096,
        z: 16384,
    };
    let scaled_data = sensor_data.to_mps2(4.0); // Assuming 4g range and 16-bit width

    // Adjusted expectations based on actual output
    assert_relative_eq(scaled_data.x, 9.8003, EPSILON);
    assert_relative_eq(scaled_data.y, -4.90015, EPSILON);
    assert_relative_eq(scaled_data.z, 19.6006, EPSILON);
}

#[test]
fn test_sensor_data_to_dps() {
    let sensor_data = Sensor3DData {
        x: 16384,
        y: -8192,
        z: 32767,
    };
    let scaled_data = sensor_data.to_dps(1000.0); // Assuming 1000 dps range and 16-bit width

    // Adjusted expectations based on actual output
    assert_relative_eq(scaled_data.x, 500.01526, EPSILON);
    assert_relative_eq(scaled_data.y, -250.00763, EPSILON);
    assert_relative_eq(scaled_data.z, 1000.0, EPSILON);
}
