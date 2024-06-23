#![no_std]

pub mod device;
mod interface;
mod registers;
pub use registers::Register;
mod types;
pub use types::{
    AccelerometerPowerMode, AccelerometerRange, Data, DataScaled, Error, GyroscopePowerMode,
    GyroscopeRange, SensorPowerMode, SensorSelector, Status,
};
mod sensor_data;
pub use sensor_data::*;

pub struct Bmi323<DI, D> {
    iface: DI,
    delay: D,
    accel_range: AccelerometerRange,
    gyro_range: GyroscopeRange,
}

pub struct AccelerometerConfig {
    pub odr: u8,
    pub range: AccelerometerRange,
    pub bw: u8,
    pub avg_num: u8,
    pub mode: u8,
}

pub struct GyroscopeConfig {
    pub odr: u8,
    pub range: GyroscopeRange,
    pub bw: u8,
    pub avg_num: u8,
    pub mode: u8,
}

pub struct AccelerometerData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

pub struct GyroscopeData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl AccelerometerConfig {
    pub fn default() -> Self {
        AccelerometerConfig {
            odr: 0x08, // 100 Hz
            range: AccelerometerRange::G8,
            bw: 0x00,      // ODR/2
            avg_num: 0x00, // Average 1 sample
            mode: 0x07,    // High performance mode
        }
    }
}

impl GyroscopeConfig {
    pub fn default() -> Self {
        GyroscopeConfig {
            odr: 0x08, // 100 Hz
            range: GyroscopeRange::DPS2000,
            bw: 0x00,      // ODR/2
            avg_num: 0x00, // Average 1 sample
            mode: 0x07,    // High performance mode
        }
    }
}
