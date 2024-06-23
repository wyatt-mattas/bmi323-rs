pub struct Register;
impl Register {
    pub const CHIPID: u8 = 0x00;
    pub const ERR_REG: u8 = 0x01;
    pub const STATUS: u8 = 0x02;
    pub const ACC_DATA_X: u8 = 0x03;
    pub const ACC_DATA_Y: u8 = 0x04;
    pub const ACC_DATA_Z: u8 = 0x05;
    pub const GYR_DATA_X: u8 = 0x06;
    pub const GYR_DATA_Y: u8 = 0x07;
    pub const GYR_DATA_Z: u8 = 0x08;
    pub const TEMP_DATA: u8 = 0x09;
    pub const SENSOR_TIME_0: u8 = 0x0A;
    pub const SENSOR_TIME_1: u8 = 0x0B;
    pub const ACC_CONF: u8 = 0x20;
    pub const GYR_CONF: u8 = 0x21;
    pub const CMD: u8 = 0x7E;
    pub const BMI323_CHIP_ID: u8 = 0x43;
    pub const CMD_SOFT_RESET: u16 = 0xDEAF;
}
