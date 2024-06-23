#[derive(Debug)]
pub enum Error<E> {
    Comm(E),
    InvalidDevice,
    InvalidConfig,
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelerometerRange {
    G2,
    G4,
    G8,
    G16,
}

impl AccelerometerRange {
    pub fn to_g(&self) -> f32 {
        match self {
            AccelerometerRange::G2 => 2.0,
            AccelerometerRange::G4 => 4.0,
            AccelerometerRange::G8 => 8.0,
            AccelerometerRange::G16 => 16.0,
        }
    }
}

impl Default for AccelerometerRange {
    fn default() -> Self {
        AccelerometerRange::G8
    }
}

impl AccelerometerRange {
    pub fn bits(self) -> u8 {
        self as u8
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GyroscopeRange {
    DPS125,
    DPS250,
    DPS500,
    DPS1000,
    DPS2000,
}

impl GyroscopeRange {
    pub fn to_dps(&self) -> f32 {
        match self {
            GyroscopeRange::DPS125 => 125.0,
            GyroscopeRange::DPS250 => 250.0,
            GyroscopeRange::DPS500 => 500.0,
            GyroscopeRange::DPS1000 => 1000.0,
            GyroscopeRange::DPS2000 => 2000.0,
        }
    }
}

impl Default for GyroscopeRange {
    fn default() -> Self {
        GyroscopeRange::DPS2000
    }
}

impl GyroscopeRange {
    pub fn bits(self) -> u8 {
        self as u8
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
