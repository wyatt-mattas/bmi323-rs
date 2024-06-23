use bmi323_scratch::{
    AccelerometerConfig, AccelerometerRange, Bmi323, GyroscopeConfig, GyroscopeRange,
};
use embedded_hal_mock::eh1::delay::NoopDelay as MockDelay;
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

#[test]
fn test_bmi323_init() {
    let expectations = [
        I2cTransaction::write(0x68, vec![0x7E, 0xAF, 0xDE]),
        I2cTransaction::write_read(0x68, vec![0x00], vec![0x43]),
        I2cTransaction::write(0x68, vec![0x20, 0x07]),
        I2cTransaction::write(0x68, vec![0x21, 0x07]),
        I2cTransaction::write(0x68, vec![0x21, 0x07]),
        I2cTransaction::write(0x68, vec![0x22, 0x07]),
    ];

    let i2c = I2cMock::new(&expectations);
    let delay = MockDelay::new();
    let mut bmi323 = Bmi323::new_with_i2c(i2c, 0x68, delay);

    bmi323.init().unwrap();
}

#[test]
fn test_bmi323_set_accel_config() {
    let expectations = [
        I2cTransaction::write(0x68, vec![0x20, 0x07]),
        I2cTransaction::write(0x68, vec![0x21, 0x07]),
    ];

    let i2c = I2cMock::new(&expectations);
    let delay = MockDelay::new();
    let mut bmi323 = Bmi323::new_with_i2c(i2c, 0x68, delay);

    let config = AccelerometerConfig {
        odr: 0x08, // 100Hz
        range: AccelerometerRange::G16,
        bw: 0x01,
        avg_num: 0x00,
        mode: 0x00, // Normal mode
    };

    bmi323.set_accel_config(config).unwrap();
}

#[test]
fn test_bmi323_set_gyro_config() {
    let expectations = [
        I2cTransaction::write(0x68, vec![0x21, 0x07]),
        I2cTransaction::write(0x68, vec![0x22, 0x07]),
    ];

    let i2c = I2cMock::new(&expectations);
    let delay = MockDelay::new();
    let mut bmi323 = Bmi323::new_with_i2c(i2c, 0x68, delay);

    let config = GyroscopeConfig {
        odr: 0x08, // 100Hz
        range: GyroscopeRange::DPS2000,
        bw: 0x00,
        avg_num: 0x00,
        mode: 0x00, // Normal mode
    };

    bmi323.set_gyro_config(config).unwrap();
}
