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
    NullPtr = -1,
    ComFail = -2,
    DevNotFound = -3,
    AccInvalidCfg = -4,
    GyroInvalidCfg = -5,
    InvalidSensor = -6,
    InvalidIntPin = -7,
    InvalidInput = -8,
    InvalidStatus = -9,
    DataRdyIntFailed = -10,
    InvalidFocPosition = -11,
    InvalidStSelection = -12,
    OutOfRange = -13,
    FeatureEngineStatus = -14,
}