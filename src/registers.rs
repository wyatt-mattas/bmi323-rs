/// BMI323 register addresses and constant values
pub struct Register;
impl Register {
    /// Chip ID register address
    pub const CHIPID: u8 = 0x00;
    /// Error register address
    pub const ERR_REG: u8 = 0x01;
    /// Status register address
    pub const STATUS: u8 = 0x02;
    /// Accelerometer X-axis data register address
    pub const ACC_DATA_X: u8 = 0x03;
    /// Accelerometer Y-axis data register address
    pub const ACC_DATA_Y: u8 = 0x04;
    /// Accelerometer Z-axis data register address
    pub const ACC_DATA_Z: u8 = 0x05;
    /// Gyroscope X-axis data register address
    pub const GYR_DATA_X: u8 = 0x06;
    /// Gyroscope Y-axis data register address
    pub const GYR_DATA_Y: u8 = 0x07;
    /// Gyroscope Z-axis data register address
    pub const GYR_DATA_Z: u8 = 0x08;
    /// Temperature data register address
    pub const TEMP_DATA: u8 = 0x09;
    /// Sensor time register address (lower byte)
    pub const SENSOR_TIME_0: u8 = 0x0A;
    /// Sensor time register address (upper byte)
    pub const SENSOR_TIME_1: u8 = 0x0B;
    /// Accelerometer configuration register address
    pub const ACC_CONF: u8 = 0x20;
    /// Gyroscope configuration register address
    pub const GYR_CONF: u8 = 0x21;
    /// Command register address
    pub const CMD: u8 = 0x7E;
    /// Expected chip ID for BMI323
    pub const BMI323_CHIP_ID: u8 = 0x43;
    /// Soft reset command value
    pub const CMD_SOFT_RESET: u16 = 0xDEAF;
    pub const FEATURE_IO2: u8 = 0x12;
    pub const FEATURE_IO_STATUS: u8 = 0x14;
    pub const FEATURE_CTRL: u8 = 0x40;
    pub const FEATURE_IO1: u8 = 0x11;
}
