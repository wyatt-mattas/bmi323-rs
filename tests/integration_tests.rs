use bmi323::{Bmi323, Register, SensorConfig};
use embedded_hal_mock::eh1::delay::NoopDelay as MockDelay;
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

#[test]
fn test_bmi323_init() {
    let expectations = [
        I2cTransaction::write(0x68, vec![0x7E, 0xAF, 0xDE]),
        I2cTransaction::write_read(0x68, vec![0x00], vec![0x43]),
    ];

    let i2c = I2cMock::new(&expectations);
    let delay = MockDelay::new();
    let mut bmi323 = Bmi323::new_with_i2c(i2c, 0x68, delay);

    bmi323.init().unwrap();

    bmi323.destroy().done();
}

#[test]
fn test_bmi323_set_sensor_config() {
    let expectations = [
        I2cTransaction::write(0x68, vec![0x20, 0xB8, 0x40]),
        I2cTransaction::write(0x68, vec![0x21, 0x48, 0x40]),
    ];

    let i2c = I2cMock::new(&expectations);
    let delay = MockDelay::new();
    let mut bmi323 = Bmi323::new_with_i2c(i2c, 0x68, delay);

    let accel_config = SensorConfig {
        odr: 0x08,   // 100Hz
        range: 0x03, // G16 for accelerometer
        bw: 0x01,
        avg_num: 0x00,
        mode: 0x04, // Normal performance mode
    };

    let gyro_config = SensorConfig {
        odr: 0x08,
        range: 0x04,
        bw: 0x00,
        avg_num: 0x00,
        mode: 0x04,
    };

    bmi323
        .set_sensor_config(Register::ACC_CONF, accel_config)
        .unwrap();

    bmi323
        .set_sensor_config(Register::GYR_CONF, gyro_config)
        .unwrap();

    bmi323.destroy().done();
}

#[test]
fn test_bmi323_read_sensor_data() {
    let expectations = [I2cTransaction::write_read(
        0x68,
        vec![0x03],
        vec![0, 0, 0, 0, 0, 0],
    )];

    let i2c = I2cMock::new(&expectations);
    let delay = MockDelay::new();
    let mut bmi323 = Bmi323::new_with_i2c(i2c, 0x68, delay);

    let sensor_data = bmi323.read_sensor_data(Register::ACC_DATA_X).unwrap();
    assert_eq!(sensor_data.x, 0);
    assert_eq!(sensor_data.y, 0);
    assert_eq!(sensor_data.z, 0);

    bmi323.destroy().done();
}
