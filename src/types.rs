use core::fmt::Debug;
use micromath::F32Ext;

#[derive(Debug)]
pub enum Error<E> {
    Comm(E),
    InvalidDevice,
    InvalidConfig,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct SensorPowerMode {
    pub accel: AccelerometerPowerMode,
    pub gyro: GyroscopePowerMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum AccelerometerPowerMode {
    Normal,
    Suspend,
    LowPower,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelerometerRange {
    G2 = 0,
    G4 = 1,
    G8 = 2,
    G16 = 3,
}

impl AccelerometerRange {
    pub fn to_g(self) -> f32 {
        2.0f32.powi(self as i32 + 1)
    }
}

impl Default for AccelerometerRange {
    fn default() -> Self {
        AccelerometerRange::G8
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum GyroscopePowerMode {
    Normal,
    Suspend,
    FastStartUp,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GyroscopeRange {
    DPS125 = 0,
    DPS250 = 1,
    DPS500 = 2,
    DPS1000 = 3,
    DPS2000 = 4,
}

impl GyroscopeRange {
    pub fn to_dps(self) -> f32 {
        125.0 * 2.0f32.powi(self as i32)
    }
}

impl Default for GyroscopeRange {
    fn default() -> Self {
        GyroscopeRange::DPS2000
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct Status {
    pub accel_data_ready: bool,
    pub gyro_data_ready: bool,
    pub nvm_ready: bool,
    pub foc_ready: bool,
    pub gyro_self_test_ok: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct SensorSelector {
    pub(crate) accel: bool,
    pub(crate) gyro: bool,
    pub(crate) time: bool,
}

impl SensorSelector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn accel(mut self) -> Self {
        self.accel = true;
        self
    }

    pub fn gyro(mut self) -> Self {
        self.gyro = true;
        self
    }

    pub fn time(mut self) -> Self {
        self.time = true;
        self
    }

    pub fn all() -> Self {
        Self {
            accel: true,
            gyro: true,
            time: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct Sensor3DData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct Data {
    pub accel: Option<Sensor3DData>,
    pub gyro: Option<Sensor3DData>,
    pub time: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct Sensor3DDataScaled {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct DataScaled {
    pub accel: Option<Sensor3DDataScaled>,
    pub gyro: Option<Sensor3DDataScaled>,
    pub time: Option<u32>,
}

impl From<AccelerometerRange> for u8 {
    fn from(range: AccelerometerRange) -> Self {
        range as u8
    }
}

impl From<GyroscopeRange> for u8 {
    fn from(range: GyroscopeRange) -> Self {
        range as u8
    }
}

impl AccelerometerRange {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => AccelerometerRange::G2,
            1 => AccelerometerRange::G4,
            2 => AccelerometerRange::G8,
            3 => AccelerometerRange::G16,
            _ => AccelerometerRange::G8, // Default to G8 for invalid values
        }
    }
}

impl GyroscopeRange {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => GyroscopeRange::DPS125,
            1 => GyroscopeRange::DPS250,
            2 => GyroscopeRange::DPS500,
            3 => GyroscopeRange::DPS1000,
            4 => GyroscopeRange::DPS2000,
            _ => GyroscopeRange::DPS2000, // Default to DPS2000 for invalid values
        }
    }
}
