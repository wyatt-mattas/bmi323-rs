#![no_std]

mod bmi3_core;
pub mod bmi3_defs;
pub mod bmi3_macros;
pub mod bmi3_sens_axes;
mod bmi3_types;
pub mod bmi3dev;
pub mod bmi3mapint;
pub mod config;
pub mod device;
pub mod enums;
pub mod error;
mod get_sensor_config;
mod interface;
mod set_accel_sensor_config;
mod set_gyro_sensor_config;
pub mod types;

//TODO implement i2cerror struct instead of using bmi3error for everything
mod private {
    use super::interface;
    pub trait Sealed {}

    impl<SPI> Sealed for interface::SpiInterface<SPI> {}
    impl<I2C> Sealed for interface::I2cInterface<I2C> {}
}

pub struct Bmi323<DI> {
    intf: DI,
    // TODO add accel and gyro range
}
