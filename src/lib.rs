#![no_std]

//mod bmi3_core;
//pub mod bmi3_defs;
//pub mod bmi3_macros;
//pub mod bmi3_sens_axes;
//mod bmi3_types;
//pub mod bmi3dev;
//pub mod bmi3mapint;
//pub mod config;
pub mod device;
//pub mod enums;
//mod get_sensor_config;
mod interface;
//mod set_accel_sensor_config;
//mod set_gyro_sensor_config;
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
