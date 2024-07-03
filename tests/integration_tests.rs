use bmi323::{
    AccelConfig, AccelerometerPowerMode, AccelerometerRange, AverageNum, Bandwidth, Bmi323,
    GyroConfig, GyroscopePowerMode, GyroscopeRange, OutputDataRate,
};
use embedded_hal_mock::eh1::delay::NoopDelay as MockDelay;
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

#[test]
fn test_bmi323_init() {
    let expectations = [
        I2cTransaction::write(0x68, vec![0x7E, 0xAF, 0xDE]),
        I2cTransaction::write_read(0x68, vec![0x00], vec![0x43]),
    ];

    let mut i2c = I2cMock::new(&expectations);
    let delay = MockDelay::new();
    let mut bmi323 = Bmi323::new_with_i2c(i2c.clone(), 0x68, delay);

    bmi323.init().unwrap();

    i2c.done();
}

#[test]
fn test_bmi323_set_sensor_config() {
    let expectations = [
        I2cTransaction::write(0x68, vec![0x20, 0xB8, 0x46]), // Accelerometer config
        I2cTransaction::write(0x68, vec![0x21, 0x48, 0x46]), // Gyroscope config
    ];

    let mut i2c = I2cMock::new(&expectations);
    let delay = MockDelay::new();
    let mut bmi323 = Bmi323::new_with_i2c(i2c.clone(), 0x68, delay);

    let accel_config = AccelConfig::builder()
        .odr(OutputDataRate::Odr100hz)
        .range(AccelerometerRange::G16)
        .bw(Bandwidth::OdrQuarter) // ODR/4
        .avg_num(AverageNum::Avg64)
        .mode(AccelerometerPowerMode::Normal)
        .build();

    let gyro_config = GyroConfig::builder()
        .odr(OutputDataRate::Odr100hz)
        .range(GyroscopeRange::DPS2000)
        .bw(Bandwidth::OdrHalf) // ODR/2
        .avg_num(AverageNum::Avg64)
        .mode(GyroscopePowerMode::Normal)
        .build();

    bmi323.set_accel_config(accel_config).unwrap();
    bmi323.set_gyro_config(gyro_config).unwrap();

    i2c.done();
}

#[test]
fn test_bmi323_read_sensor_data() {
    let expectations = [I2cTransaction::write_read(
        0x68,
        vec![0x03],
        vec![0, 0, 0, 0, 0, 0],
    )];

    let mut i2c = I2cMock::new(&expectations);
    let delay = MockDelay::new();
    let mut bmi323 = Bmi323::new_with_i2c(i2c.clone(), 0x68, delay);

    let sensor_data = bmi323.read_accel_data().unwrap();
    assert_eq!(sensor_data.x, 0);
    assert_eq!(sensor_data.y, 0);
    assert_eq!(sensor_data.z, 0);

    i2c.done();
}
