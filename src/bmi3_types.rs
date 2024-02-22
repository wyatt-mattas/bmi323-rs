use crate::enums::{Bmi3Error, I2cError};

pub type Bmi3Result<T> = core::result::Result<T, Bmi3Error>;
pub type I2cResult<T> = Result<T, I2cError>;