use crate::types::{Sensor3DData, Sensor3DDataScaled};

/// Standard gravity in m/s^2
pub const GRAVITY: f32 = 9.8;
/// Alias for accelormeter data
pub type AccelerometerData = Sensor3DData;
/// Alias for gyroscope data
pub type GyroscopeData = Sensor3DData;

impl Sensor3DData {
    /// Convert raw sensor data to scaled values
    ///
    /// # Arguments
    ///
    /// * `scale` - The full scale value
    /// * `half_scale` - Half of the full scale value
    fn to_scaled(&self, scale: f32, half_scale: f32) -> Sensor3DDataScaled {
        Sensor3DDataScaled {
            x: Self::lsb_to_scaled(self.x, scale, half_scale),
            y: Self::lsb_to_scaled(self.y, scale, half_scale),
            z: Self::lsb_to_scaled(self.z, scale, half_scale),
        }
    }

    /// Convert raw LSB value to scaled value
    ///
    /// # Arguments
    ///
    /// * `val` - Raw LSB value
    /// * `scale` - The full scale value
    /// * `half_scale` - Half of the full scale value
    fn lsb_to_scaled(val: i16, scale: f32, half_scale: f32) -> f32 {
        (scale * val as f32) / half_scale
    }

    /// Convert raw accelerometer data to m/s^2
    ///
    /// # Arguments
    ///
    /// * `g` - The G-force value for the current range setting
    pub fn to_mps2(&self, g: f32) -> Sensor3DDataScaled {
        self.to_scaled(GRAVITY * g, f32::from(i16::MAX))
    }

    /// Convert raw gyroscope data to degrees per second
    ///
    /// # Arguments
    ///
    /// * `dps` - The degrees per second value for the current range setting
    pub fn to_dps(&self, dps: f32) -> Sensor3DDataScaled {
        self.to_scaled(dps, f32::from(i16::MAX))
    }
}
