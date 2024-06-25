#![no_std]

pub mod device;
mod interface;
mod registers;
pub use registers::Register;
mod types;
pub use types::{
    AccelerometerPowerMode, AccelerometerRange, Data, DataScaled, Error, GyroscopePowerMode,
    GyroscopeRange, Sensor3DData, SensorPowerMode, SensorSelector, Status,
};
mod sensor_data;
pub use sensor_data::*;

pub struct Bmi323<DI, D> {
    iface: DI,
    delay: D,
    accel_range: AccelerometerRange,
    gyro_range: GyroscopeRange,
}

#[derive(Debug, Clone, Copy)]
pub struct SensorConfig {
    pub odr: u8,
    pub range: u8,
    pub bw: u8,
    pub avg_num: u8,
    pub mode: u8,
}

impl Default for SensorConfig {
    fn default() -> Self {
        SensorConfig {
            odr: 0x08,     // 100 Hz
            range: 0x02,   // G8 for accel, DPS2000 for gyro
            bw: 0x00,      // ODR/2
            avg_num: 0x00, // Average 1 sample
            mode: 0x07,    // High performance mode
        }
    }
}
