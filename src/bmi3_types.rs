use crate::enums::{Bmi3Error, I2cError};

pub type Bmi3Result<T> = core::result::Result<T, Bmi3Error>;
pub type I2cResult<T> = Result<T, I2cError>;

/// All possible errors in this crate
#[derive(Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Error<CommE> {
    /// I²C / SPI communication error
    Comm(CommE),
    /// Invalid input data provided
    InvalidInputData,
}
