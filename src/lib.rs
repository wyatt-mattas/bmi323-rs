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

pub struct Bmi323<DI> {
    iface: DI,
    accel_range: AccelerometerRange,
    gyro_range: GyroscopeRange,
}
