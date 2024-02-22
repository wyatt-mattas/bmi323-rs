#![no_std]

pub mod bmi3_defs;
pub mod bmi3dev;
pub mod bmi3mapint;
pub mod config;
pub mod enums;
pub mod bmi3_macros;
mod bmi3_types;
mod set_accel_sensor_config;
mod get_sensor_config;
mod bmi3_core;
mod set_gyro_sensor_config;
pub mod bmi3_sens_axes;

//TODO implement i2cerror struct instead of using bmi3error for everything