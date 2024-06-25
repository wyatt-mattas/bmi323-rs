use crate::types::{Sensor3DData, Sensor3DDataScaled};

pub const GRAVITY: f32 = 9.8;

pub type AccelerometerData = Sensor3DData;
pub type GyroscopeData = Sensor3DData;

impl Sensor3DData {
    pub fn to_scaled(&self, scale: f32, half_scale: f32) -> Sensor3DDataScaled {
        Sensor3DDataScaled {
            x: Self::lsb_to_scaled(self.x, scale, half_scale),
            y: Self::lsb_to_scaled(self.y, scale, half_scale),
            z: Self::lsb_to_scaled(self.z, scale, half_scale),
        }
    }

    fn lsb_to_scaled(val: i16, scale: f32, half_scale: f32) -> f32 {
        (scale * val as f32) / half_scale
    }

    pub fn to_mps2(&self, g: f32) -> Sensor3DDataScaled {
        self.to_scaled(GRAVITY * g, f32::from(i16::MAX))
    }

    pub fn to_dps(&self, dps: f32) -> Sensor3DDataScaled {
        self.to_scaled(dps, f32::from(i16::MAX))
    }
}
