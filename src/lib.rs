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
    AccelerometerPowerMode, AccelerometerRange, AverageNum, Bandwidth, Error, GyroscopePowerMode,
    GyroscopeRange, OutputDataRate, Sensor3DData, Sensor3DDataScaled,
};
mod sensor_data;
pub use sensor_data::*;

/// Main struct representing the BMI323 device
pub struct Bmi323<DI, D> {
    /// Communication interface (I2C or SPI)
    iface: DI,
    /// Delay provider
    delay: D,
    /// Current accelerometer range
    accel_range: AccelerometerRange,
    /// Current gyroscope range
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
    /// Create a new AccelConfigBuilder
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

/// Builder for AccelConfig
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
    /// Set the output data rate
    pub fn odr(mut self, odr: OutputDataRate) -> Self {
        self.odr = Some(odr);
        self
    }

    /// Set the measurement range
    pub fn range(mut self, range: AccelerometerRange) -> Self {
        self.range = Some(range);
        self
    }

    /// Set the bandwidth
    pub fn bw(mut self, bw: Bandwidth) -> Self {
        self.bw = Some(bw);
        self
    }

    /// Set the number of samples to average
    pub fn avg_num(mut self, avg_num: AverageNum) -> Self {
        self.avg_num = Some(avg_num);
        self
    }

    /// Set the power mode
    pub fn mode(mut self, mode: AccelerometerPowerMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Build the AccelConfig
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
    /// Create a new GyroConfigBuilder
    pub fn builder() -> GyroConfigBuilder {
        GyroConfigBuilder::default()
    }
}

/// Builder for GyroConfig
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
    /// Set the output data rate
    pub fn odr(mut self, odr: OutputDataRate) -> Self {
        self.odr = Some(odr);
        self
    }

    /// Set the measurement range
    pub fn range(mut self, range: GyroscopeRange) -> Self {
        self.range = Some(range);
        self
    }

    /// Set the bandwidth
    pub fn bw(mut self, bw: Bandwidth) -> Self {
        self.bw = Some(bw);
        self
    }

    /// Set the power mode
    pub fn avg_num(mut self, avg_num: AverageNum) -> Self {
        self.avg_num = Some(avg_num);
        self
    }

    /// Set the power mode
    pub fn mode(mut self, mode: GyroscopePowerMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Build the GyroConfig
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
    /// Convert AccelConfig to a 16-bit register value
    fn from(config: AccelConfig) -> Self {
        (config.odr as u16 & 0x0F)
            | ((config.range as u16 & 0x07) << 4)
            | ((config.bw as u16 & 0x01) << 7)
            | ((config.avg_num as u16 & 0x07) << 8)
            | ((config.mode as u16 & 0x07) << 12)
    }
}

impl From<GyroConfig> for u16 {
    /// Convert GyroConfig to a 16-bit register value
    fn from(config: GyroConfig) -> Self {
        (config.odr as u16 & 0x0F)
            | ((config.range as u16 & 0x07) << 4)
            | ((config.bw as u16 & 0x01) << 7)
            | ((config.avg_num as u16 & 0x07) << 8)
            | ((config.mode as u16 & 0x07) << 12)
    }
}
