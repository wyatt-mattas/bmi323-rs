use crate::config::Bmi3GyroConfig;

use crate::config::Bmi3AccelConfig;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SensorType {
    Accel, // Example values, adjust according to your device's documentation
    Gyro,
}

#[derive(Debug, Clone, Copy)]
pub enum Bmi3SensConfigTypes {
    Accel(Bmi3AccelConfig),
    Gyro(Bmi3GyroConfig),
}

pub enum Bmi3HwIntPin {
    Bmi3IntNone = 0,
    Bmi3Int1 = 1,
    Bmi3Int2 = 2,
    Bmi3I3cInt = 3,
    Bmi3IntPinMax = 4,
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bmi3Intf {
    Spi,
    I2c,
    I3c, // Extend with other interfaces as needed
}

#[derive(Debug, PartialEq, Eq)]
pub enum Bmi3Error {
    NullPtr,
    ComFail,
    DevNotFound,
    AccInvalidCfg,
    GyroInvalidCfg,
    InvalidSensor,
    InvalidIntPin,
    InvalidInput,
    InvalidStatus,
    DataRdyIntFailed,
    InvalidFocPosition,
    InvalidStSelection,
    OutOfRange,
    FeatureEngineStatus,
}
#[derive(Debug, PartialEq, Eq)]
pub enum I2cError {
    NullPtr,
    ComFail,
    NackReceived,
    InitializationError,
}
