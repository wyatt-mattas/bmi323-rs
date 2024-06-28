#![no_std]

/// BMI323 driver for Rust
///
/// This module provides a high-level interface for interacting with the Bosch BMI323 IMU.
/// It supports both I2C and SPI interfaces and allows for configuration of accelerometer
/// and gyroscope settings.
pub mod device;
mod interface;
mod registers;
pub use registers::Register;
mod types;
pub use types::{
    AccelerometerPowerMode, AccelerometerRange, AverageNum, Bandwidth, Data, DataScaled, Error,
    GyroscopePowerMode, GyroscopeRange, OutputDataRate, Sensor3DData, Status,
};
mod sensor_data;
pub use sensor_data::*;

pub struct Bmi323<DI, D> {
    iface: DI,
    delay: D,
    accel_range: AccelerometerRange,
    gyro_range: GyroscopeRange,
}

/// Configuration for the accelerometer
#[derive(Debug, Clone, Copy)]
pub struct AccelConfig {
    /// Output data rate
    pub odr: OutputDataRate,
    /// Measurement range
    pub range: AccelerometerRange,
    /// Bandwidth
    pub bw: Bandwidth,
    /// Number of samples to average
    pub avg_num: AverageNum,
    /// Power mode
    pub mode: AccelerometerPowerMode,
}

impl AccelConfig {
    pub fn builder() -> AccelConfigBuilder {
        AccelConfigBuilder::default()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AccelConfigBuilder {
    odr: Option<OutputDataRate>,
    range: Option<AccelerometerRange>,
    bw: Option<Bandwidth>,
    avg_num: Option<AverageNum>,
    mode: Option<AccelerometerPowerMode>,
}

impl Default for AccelConfigBuilder {
    fn default() -> Self {
        Self {
            odr: None,
            range: None,
            bw: None,
            avg_num: None,
            mode: None,
        }
    }
}

impl AccelConfigBuilder {
    pub fn odr(mut self, odr: OutputDataRate) -> Self {
        self.odr = Some(odr);
        self
    }

    pub fn range(mut self, range: AccelerometerRange) -> Self {
        self.range = Some(range);
        self
    }

    pub fn bw(mut self, bw: Bandwidth) -> Self {
        self.bw = Some(bw);
        self
    }

    pub fn avg_num(mut self, avg_num: AverageNum) -> Self {
        self.avg_num = Some(avg_num);
        self
    }

    pub fn mode(mut self, mode: AccelerometerPowerMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn build(self) -> AccelConfig {
        AccelConfig {
            odr: self.odr.unwrap_or(OutputDataRate::Odr100hz),
            range: self.range.unwrap_or(AccelerometerRange::G8),
            bw: self.bw.unwrap_or(Bandwidth::OdrQuarter),
            avg_num: self.avg_num.unwrap_or(AverageNum::Avg1),
            mode: self.mode.unwrap_or(AccelerometerPowerMode::Normal),
        }
    }
}

/// Configuration for the gyroscope
#[derive(Debug, Clone, Copy)]
pub struct GyroConfig {
    /// Output data rate
    pub odr: OutputDataRate,
    /// Measurement range
    pub range: GyroscopeRange,
    /// Bandwidth
    pub bw: Bandwidth,
    /// Number of samples to average
    pub avg_num: AverageNum,
    /// Power mode
    pub mode: GyroscopePowerMode,
}

impl GyroConfig {
    pub fn builder() -> GyroConfigBuilder {
        GyroConfigBuilder::default()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GyroConfigBuilder {
    odr: Option<OutputDataRate>,
    range: Option<GyroscopeRange>,
    bw: Option<Bandwidth>,
    avg_num: Option<AverageNum>,
    mode: Option<GyroscopePowerMode>,
}

impl Default for GyroConfigBuilder {
    fn default() -> Self {
        Self {
            odr: None,
            range: None,
            bw: None,
            avg_num: None,
            mode: None,
        }
    }
}

impl GyroConfigBuilder {
    pub fn odr(mut self, odr: OutputDataRate) -> Self {
        self.odr = Some(odr);
        self
    }

    pub fn range(mut self, range: GyroscopeRange) -> Self {
        self.range = Some(range);
        self
    }

    pub fn bw(mut self, bw: Bandwidth) -> Self {
        self.bw = Some(bw);
        self
    }

    pub fn avg_num(mut self, avg_num: AverageNum) -> Self {
        self.avg_num = Some(avg_num);
        self
    }

    pub fn mode(mut self, mode: GyroscopePowerMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn build(self) -> GyroConfig {
        GyroConfig {
            odr: self.odr.unwrap_or(OutputDataRate::Odr100hz),
            range: self.range.unwrap_or(GyroscopeRange::DPS2000),
            bw: self.bw.unwrap_or(Bandwidth::OdrHalf),
            avg_num: self.avg_num.unwrap_or(AverageNum::Avg1),
            mode: self.mode.unwrap_or(GyroscopePowerMode::Normal),
        }
    }
}

impl From<AccelConfig> for u16 {
    fn from(config: AccelConfig) -> Self {
        (config.odr as u16 & 0x0F)
            | ((config.range as u16 & 0x07) << 4)
            | ((config.bw as u16 & 0x01) << 7)
            | ((config.avg_num as u16 & 0x07) << 8)
            | ((config.mode as u16 & 0x07) << 12)
    }
}

impl From<GyroConfig> for u16 {
    fn from(config: GyroConfig) -> Self {
        (config.odr as u16 & 0x0F)
            | ((config.range as u16 & 0x07) << 4)
            | ((config.bw as u16 & 0x01) << 7)
            | ((config.avg_num as u16 & 0x07) << 8)
            | ((config.mode as u16 & 0x07) << 12)
    }
}
