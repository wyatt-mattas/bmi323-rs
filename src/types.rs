#[derive(Debug)]
pub enum Error<E> {
    Comm(E),
    Other,
}

/// Sensor power mode
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct SensorPowerMode {
    /// Accelerometer power mode
    pub accel: AccelerometerPowerMode,
    /// Gyroscope power mode
    pub gyro: GyroscopePowerMode,
}

/// Accelerometer power mode
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum AccelerometerPowerMode {
    /// Normal mode
    Normal,
    /// Suspend mode
    Suspend,
    /// Low power mode
    LowPower,
}

// TODO update these with either Register values or hex values
/// Accelerometer Range
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum AccelerometerRange {
    /// +- 2G
    #[default]
    G2 = 0b0000_0011,
    /// +- 4G
    G4 = 0b0000_0101,
    /// +- 8G
    G8 = 0b0000_1000,
}

impl AccelerometerRange {
    pub(crate) fn multiplier(self) -> f32 {
        match self {
            AccelerometerRange::G2 => 1. / 16384.,
            AccelerometerRange::G4 => 1. / 8192.,
            AccelerometerRange::G8 => 1. / 4096.,
        }
    }
}

/// Gyroscope power mode
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum GyroscopePowerMode {
    /// Normal mode
    Normal,
    /// Suspend mode
    Suspend,
    /// Fast start-up mode
    FastStartUp,
}

// TODO update these with either Register values or hex values
/// Gyroscope range
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum GyroscopeRange {
    /// 16.4 LSB/°/s <-> 61.0 m°/s / LSB
    #[default]
    Scale2000 = 0b0000_0000,
    /// 32.8 LSB/°/s <-> 30.5 m°/s / LSB
    Scale1000 = 0b0000_0001,
    /// 65.6 LSB/°/s <-> 15.3 m°/s / LSB
    Scale500 = 0b0000_0010,
    /// 131.2 LSB/°/s <-> 7.6 m°/s / LSB
    Scale250 = 0b0000_0011,
    /// 262.4 LSB/°/s  3.8m°/s / LSB
    Scale125 = 0b0000_0100,
}

impl GyroscopeRange {
    pub(crate) fn multiplier(self) -> f32 {
        match self {
            GyroscopeRange::Scale2000 => 1. / 16.4,
            GyroscopeRange::Scale1000 => 1. / 32.8,
            GyroscopeRange::Scale500 => 1. / 65.6,
            GyroscopeRange::Scale250 => 1. / 131.2,
            GyroscopeRange::Scale125 => 1. / 262.4,
        }
    }
}

/// Sensor status flags
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct Status {
    /// Accelerometer has data ready
    pub accel_data_ready: bool,
    /// Gyroscope has data ready
    pub gyro_data_ready: bool,
    /// NVM controller ready
    pub nvm_ready: bool,
    /// Fast offset compensation (FOC) completed
    pub foc_ready: bool,
    /// Gyroscope self-test completed successfully
    pub gyro_self_test_ok: bool,
}

/// Sensor data read selector
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct SensorSelector {
    pub(crate) accel: bool,
    pub(crate) gyro: bool,
    pub(crate) time: bool,
}

impl SensorSelector {
    /// Create new instance of the selector.
    ///
    /// This does not include any data.
    pub fn new() -> Self {
        SensorSelector {
            accel: false,
            gyro: false,
            time: false,
        }
    }

    /// Include acceleration sensor data
    pub fn accel(mut self) -> Self {
        self.accel = true;
        self
    }

    /// Include gyroscope sensor data
    pub fn gyro(mut self) -> Self {
        self.gyro = true;
        self
    }

    /// Include sensor time
    pub fn time(mut self) -> Self {
        self.time = true;
        self
    }

    /// Include accelerometer, gyroscope, magnetometer and time data
    pub fn all() -> Self {
        SensorSelector {
            accel: true,
            gyro: true,
            time: true,
        }
    }
}

impl Default for SensorSelector {
    fn default() -> Self {
        SensorSelector::all()
    }
}

/// Sensor data read selector
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct Sensor3DData {
    /// X axis data
    pub x: i16,
    /// Y axis data
    pub y: i16,
    /// Z axis data
    pub z: i16,
}

/// Sensor data read
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct Data {
    /// Accelerometer data (if selected)
    pub accel: Option<Sensor3DData>,
    /// Gyroscope data (if selected)
    pub gyro: Option<Sensor3DData>,
    /// Time data (if selected)
    pub time: Option<u32>,
}

/// Floating point 3D data
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct Sensor3DDataScaled {
    /// X axis data
    pub x: f32,
    /// Y axis data
    pub y: f32,
    /// Z axis data
    pub z: f32,
}

/// Sensor data read
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct DataScaled {
    /// Accelerometer data (if selected)
    pub accel: Option<Sensor3DDataScaled>,
    /// Gyroscope data (if selected)
    pub gyro: Option<Sensor3DDataScaled>,
    /// Time data (if selected)
    pub time: Option<u32>,
}
