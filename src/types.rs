#[derive(Debug, Clone, Copy)]
pub enum AccelerometerRange {
    G2,
    G4,
    G8,
    G16,
}

impl Default for AccelerometerRange {
    fn default() -> Self {
        AccelerometerRange::G2
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GyroscopeRange {
    Dps2000,
    Dps1000,
    Dps500,
    Dps250,
    Dps125,
}

impl Default for GyroscopeRange {
    fn default() -> Self {
        GyroscopeRange::Dps2000
    }
}

pub enum Register {
    CHIPID = 0x00,
    ACCEL_PWR_CTRL = 0x7D,
    GYRO_PWR_CTRL = 0x7E,
    STATUS = 0x1B,
    // Add other necessary registers
}
