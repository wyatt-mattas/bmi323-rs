pub const GRAVITY: f32 = 9.8;

pub struct AccelerometerData<const N: u8> {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

pub struct GyroscopeData<const N: u8> {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl<const N: u8> AccelerometerData<N> {
    pub fn to_mps2(&self, g: f32) -> AccelerometerDataScaled {
        AccelerometerDataScaled {
            x: Self::lsb_to_mps2(self.x, g),
            y: Self::lsb_to_mps2(self.y, g),
            z: Self::lsb_to_mps2(self.z, g),
        }
    }

    const fn half_scale() -> f32 {
        (1u32 << (N - 1)) as f32
    }

    pub fn lsb_to_mps2(val: i16, g: f32) -> f32 {
        (val as f32 * g * GRAVITY) / Self::half_scale()
    }
}

impl<const N: u8> GyroscopeData<N> {
    pub fn to_dps(&self, dps: f32) -> GyroscopeDataScaled {
        GyroscopeDataScaled {
            x: Self::lsb_to_dps(self.x, dps),
            y: Self::lsb_to_dps(self.y, dps),
            z: Self::lsb_to_dps(self.z, dps),
        }
    }

    const fn half_scale() -> f32 {
        (1u32 << (N - 1)) as f32
    }

    pub fn lsb_to_dps(val: i16, dps: f32) -> f32 {
        (dps / Self::half_scale()) * (val as f32)
    }
}

pub struct AccelerometerDataScaled {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct GyroscopeDataScaled {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Lookup tables for common range values
//pub const ACCEL_RANGE_VALUES: [f32; 4] = [2.0, 4.0, 8.0, 16.0];
//pub const GYRO_RANGE_VALUES: [f32; 5] = [125.0, 250.0, 500.0, 1000.0, 2000.0];
