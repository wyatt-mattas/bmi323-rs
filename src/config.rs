use crate::enums::Bmi3SensConfigTypes;

use crate::enums::SensorType;

#[derive(Debug, Clone, Copy)]
pub struct Bmi3SensConfig {
    // Defines the type of sensor
    pub sensor_type: SensorType, // Consider using an enum for type safety if applicable

    // Defines various sensor configurations
    pub cfg: Option<Bmi3SensConfigTypes>,
}

#[derive(Debug, Clone, Copy)]
pub struct Bmi3AccelConfig {
    // Output data rate in Hz
    pub odr: u8,

    // Bandwidth parameter
    pub bwp: u8,

    // Filter accel mode
    pub acc_mode: u8,

    // Gravity range
    pub range: u8,

    // Defines the number of samples to be averaged
    pub avg_num: u8,
}

impl Bmi3AccelConfig {
    pub fn default() -> Self {
        Bmi3AccelConfig {
            odr: 0,
            bwp: 0,
            acc_mode: 0,
            range: 0,
            avg_num: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bmi3GyroConfig {
    // Output data rate in Hz
    pub odr: u8,

    // Bandwidth parameter
    pub bwp: u8,

    // Filter gyro mode
    pub gyr_mode: u8,

    // Gyroscope Range
    pub range: u8,

    // Defines the number of samples to be averaged
    pub avg_num: u8,
}

impl Bmi3GyroConfig {
    pub fn default() -> Self {
        Bmi3GyroConfig {
            odr: 0,
            bwp: 0,
            gyr_mode: 0,
            range: 0,
            avg_num: 0,
        }
    }
}

impl Bmi3SensConfig {
    pub fn new(sensor_type: SensorType) -> Self {
        let cfg = match sensor_type {
            SensorType::Accel => Some(Bmi3SensConfigTypes::Accel(Bmi3AccelConfig::default())),
            SensorType::Gyro => Some(Bmi3SensConfigTypes::Gyro(Bmi3GyroConfig::default())),
        };

        Self { sensor_type, cfg }
    }
}

pub trait SensorConfig {
    fn set_config_field(&mut self, field: &str, value: u8);
}

impl SensorConfig for Bmi3AccelConfig {
    fn set_config_field(&mut self, field: &str, value: u8) {
        match field {
            "odr" => self.odr = value,
            "bwp" => self.bwp = value,
            "acc_mode" => self.acc_mode = value,
            "range" => self.range = value,
            "avg_num" => self.avg_num = value,
            _ => {} // Optionally handle unknown fields
        }
    }
}

impl SensorConfig for Bmi3GyroConfig {
    fn set_config_field(&mut self, field: &str, value: u8) {
        match field {
            "odr" => self.odr = value,
            "bwp" => self.bwp = value,
            "gyr_mode" => self.gyr_mode = value,
            "range" => self.range = value,
            "avg_num" => self.avg_num = value,
            _ => {} // Optionally handle unknown fields
        }
    }
}