// TODO make sure to add relevant registers

// Define register addresses and other constants
pub struct Register;
impl Register {
    pub const CHIPID: u8 = 0x00;
    pub const PMU_STATUS: u8 = 0x03;
    pub const CMD: u8 = 0x7E;
    // THESE ARE NOT VALID REGISTERS!
    pub const ACCEL_PWR_CTRL: u8 = 0x00;
    pub const GYRO_PWR_CTRL: u8 = 0x00;
    pub const STATUS: u8 = 0x00;
    // Add other relevant register addresses for BMI323
}
