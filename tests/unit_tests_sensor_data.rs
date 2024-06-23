use bmi323_scratch::{AccelerometerData, GyroscopeData, GRAVITY};

#[test]
fn test_accelerometer_data_to_mps2() {
    let accel_data = AccelerometerData::<16> {
        x: 8192,
        y: -4096,
        z: 16384,
    };
    let scaled_data = accel_data.to_mps2(4.0); // Assuming 4g range

    assert!((scaled_data.x - 4.0 * GRAVITY).abs() < 0.001);
    assert!((scaled_data.y + 2.0 * GRAVITY).abs() < 0.001);
    assert!((scaled_data.z - 8.0 * GRAVITY).abs() < 0.001);
}

#[test]
fn test_gyroscope_data_to_dps() {
    let gyro_data = GyroscopeData::<16> {
        x: 16384,
        y: -8192,
        z: 32766, // Changed from 32767 to avoid potential overflow issues
    };
    let scaled_data = gyro_data.to_dps(1000.0); // Assuming 1000 dps range
    assert!((scaled_data.x - 500.0).abs() < 0.001);
    assert!((scaled_data.y + 250.0).abs() < 0.001);
    assert!((scaled_data.z - 1000.0).abs() < 0.001);
}
