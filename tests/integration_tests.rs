use bmi323_scratch::{Bmi323, Register, SensorConfig};
use embedded_hal_mock::eh1::delay::NoopDelay as MockDelay;
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

#[test]
fn test_bmi323_init() {
    let expectations = [
        I2cTransaction::write(0x68, vec![0x7E, 0xAF, 0xDE]),
        I2cTransaction::write_read(0x68, vec![0x00], vec![0x43]),
        I2cTransaction::write(0x68, vec![0x20, 0x87]),
        I2cTransaction::write(0x68, vec![0x21, 0x70]),
        I2cTransaction::write(0x68, vec![0x21, 0x87]),
        I2cTransaction::write(0x68, vec![0x22, 0x70]),
    ];

    let i2c = I2cMock::new(&expectations);
    let delay = MockDelay::new();
    let mut bmi323 = Bmi323::new_with_i2c(i2c, 0x68, delay);

    bmi323.init().unwrap();
}

#[test]
fn test_bmi323_set_sensor_config() {
    let expectations = [
        I2cTransaction::write(0x68, vec![0x20, 0x87]),
        I2cTransaction::write(0x68, vec![0x21, 0x70]),
    ];

    let i2c = I2cMock::new(&expectations);
    let delay = MockDelay::new();
    let mut bmi323 = Bmi323::new_with_i2c(i2c, 0x68, delay);

    let config = SensorConfig {
        odr: 0x08, // 100Hz
        range: 3,  // G16 for accelerometer
        bw: 0x01,
        avg_num: 0x00,
        mode: 0x07, // High performance mode
    };

    bmi323
        .set_sensor_config(Register::ACC_CONF, config)
        .unwrap();
}

#[test]
fn test_bmi323_read_sensor_data() {
    let expectations = [
        I2cTransaction::write_read(0x68, vec![0x03], vec![0, 0, 0, 0, 0, 0, 0]),
        I2cTransaction::write_read(0x68, vec![0x03], vec![0x00, 0x20, 0x00, 0x40, 0xFF, 0x7F]),
    ];

    let i2c = I2cMock::new(&expectations);
    let delay = MockDelay::new();
    let mut bmi323 = Bmi323::new_with_i2c(i2c, 0x68, delay);

    let sensor_data = bmi323.read_sensor_data(Register::ACC_DATA_X).unwrap();
    assert_eq!(sensor_data.x, 0x2000); // 8192
    assert_eq!(sensor_data.y, 0x4000); // 16384
    assert_eq!(sensor_data.z, 0x7FFF); // 32767 (maximum positive value for i16)
}
