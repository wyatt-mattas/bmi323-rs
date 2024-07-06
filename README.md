# BMI323 Rust Driver

This is a Rust driver for the Bosch BMI323 Inertial Measurement Unit (IMU). The BMI323 is a highly integrated, low power IMU that provides precise acceleration and angular rate measurements.

## Features

- Support for both I2C and SPI interfaces
- Configurable accelerometer and gyroscope settings
- Reading raw and scaled sensor data
- Error handling and device initialization

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
bmi323 = "0.1.0"  # Replace with the actual version
```

Here's a basic example of how to use the driver:

```rust
use bmi323::{Bmi323, AccelConfig, GyroConfig, OutputDataRate, AccelerometerRange, GyroscopeRange};
use embedded_hal::blocking::i2c::I2c;

fn main() {
    // Initialize your I2C or SPI interface
    let i2c = // ... initialize your I2C interface
    let delay = // ... initialize your delay provider

    // Create a new BMI323 instance
    let mut imu = Bmi323::new_with_i2c(i2c, 0x68, delay);

    // Initialize the device
    imu.init().unwrap();

    // Configure accelerometer
    let accel_config = AccelConfig::builder()
        .odr(OutputDataRate::Odr100hz)
        .range(AccelerometerRange::G8)
        .build();
    imu.set_accel_config(accel_config).unwrap();

    // Configure gyroscope
    let gyro_config = GyroConfig::builder()
        .odr(OutputDataRate::Odr100hz)
        .range(GyroscopeRange::DPS2000)
        .build();
    imu.set_gyro_config(gyro_config).unwrap();

    loop {
        // Read accelerometer data
        let accel_data = imu.read_accel_data_scaled().unwrap();
        println!("Acceleration: x={}, y={}, z={}", accel_data.x, accel_data.y, accel_data.z);

        // Read gyroscope data
        let gyro_data = imu.read_gyro_data_scaled().unwrap();
        println!("Angular velocity: x={}, y={}, z={}", gyro_data.x, gyro_data.y, gyro_data.z);
    }

}
```

## License

This project is licensed under Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## References

- [BMI323 Product Page](https://www.bosch-sensortec.com/products/motion-sensors/imus/bmi323/)
- [BMI323 Datasheet](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bmi323-ds000.pdf)
