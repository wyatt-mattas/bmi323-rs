use bmi323::{AccelerometerRange, GyroscopeRange};

#[test]
fn test_accelerometer_range_to_g() {
    assert_eq!(AccelerometerRange::G2.to_g(), 2.0);
    assert_eq!(AccelerometerRange::G4.to_g(), 4.0);
    assert_eq!(AccelerometerRange::G8.to_g(), 8.0);
    assert_eq!(AccelerometerRange::G16.to_g(), 16.0);
}

#[test]
fn test_gyroscope_range_to_dps() {
    assert_eq!(GyroscopeRange::DPS125.to_dps(), 125.0);
    assert_eq!(GyroscopeRange::DPS250.to_dps(), 250.0);
    assert_eq!(GyroscopeRange::DPS500.to_dps(), 500.0);
    assert_eq!(GyroscopeRange::DPS1000.to_dps(), 1000.0);
    assert_eq!(GyroscopeRange::DPS2000.to_dps(), 2000.0);
}

#[test]
fn test_accelerometer_range_default() {
    assert_eq!(AccelerometerRange::default(), AccelerometerRange::G8);
}

#[test]
fn test_gyroscope_range_default() {
    assert_eq!(GyroscopeRange::default(), GyroscopeRange::DPS2000);
}
