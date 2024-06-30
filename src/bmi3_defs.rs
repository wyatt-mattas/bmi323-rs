pub const BMI3_REG_CHIP_ID: u8 = 0x00;
pub const BMI3_OK: i8 = 0;
pub const BMI3_REV_ID_MASK: u8 = 0xF0;
pub const BMI3_REV_ID_POS: u8 = 4;
pub const BMI3_ENABLE: u8 = 1;
pub const BMI3_ACC_DP_OFF_XYZ_13_BIT_MASK: u16 = 0x1FFF;
pub const BMI3_ACC_DP_OFF_XYZ_14_BIT_MASK: u16 = 0x3FFF;
pub const BMI3_MAX_LEN: u8 = 128;
pub const BMI3_INTF_RET_SUCCESS: i8 = 0;

/// To define error codes
pub const BMI3_E_NULL_PTR: i8 = -1;
pub const BMI3_E_COM_FAIL: i8 = -2;
pub const BMI3_E_DEV_NOT_FOUND: i8 = -3;
pub const BMI3_E_ACC_INVALID_CFG: i8 = -4;
pub const BMI3_E_GYRO_INVALID_CFG: i8 = -5;
pub const BMI3_E_INVALID_SENSOR: i8 = -6;
pub const BMI3_E_INVALID_INT_PIN: i8 = -7;
pub const BMI3_E_INVALID_INPUT: i8 = -8;
pub const BMI3_E_INVALID_STATUS: i8 = -9;
pub const BMI3_E_DATA_RDY_INT_FAILED: i8 = -10;
pub const BMI3_E_INVALID_FOC_POSITION: i8 = -11;
pub const BMI3_E_INVALID_ST_SELECTION: i8 = -12;
pub const BMI3_E_OUT_OF_RANGE: i8 = -13;
pub const BMI3_E_FEATURE_ENGINE_STATUS: i8 = -14;

/// Mask definitions for SPI read/write address
pub const BMI3_SPI_RD_MASK: u16 = 0x80;
pub const BMI3_SPI_WR_MASK: u16 = 0x7F;

/// LSB and MSB mask definitions
pub const BMI3_SET_LOW_BYTE: u16 = 0x00FF;
pub const BMI3_SET_HIGH_BYTE: u16 = 0xFF00;
pub const BMI3_SET_LOW_NIBBLE: u8 = 0x0F;

/// Command Register
pub const BMI3_REG_CMD: u8 = 0x7E;

/// BMI3 Commands
pub const BMI3_CMD_SELF_TEST_TRIGGER: u16 = 0x0100;
pub const BMI3_CMD_SELF_CALIB_TRIGGER: u16 = 0x0101;
pub const BMI3_CMD_SELF_CALIB_ABORT: u16 = 0x0200;
pub const BMI3_CMD_I3C_TCSYNC_UPDATE: u16 = 0x0201;
pub const BMI3_CMD_AXIS_MAP_UPDATE: u16 = 0x0300;
pub const BMI3_CMD_1: u16 = 0x64AD;
pub const BMI3_CMD_2: u16 = 0xD3AC;
pub const BMI3_CMD_SOFT_RESET: u16 = 0xDEAF;

/// Soft-reset delay
pub const BMI3_SOFT_RESET_DELAY: u16 = 2000;

/// Feature engine I/O register 1.
pub const BMI3_REG_FEATURE_IO2: u8 = 0x12;

/// Feature I/O synchronization status and trigger.
pub const BMI3_REG_FEATURE_IO_STATUS: u8 = 0x14;

/// Feature engine control register
pub const BMI3_REG_FEATURE_CTRL: u8 = 0x40;

/// Feature engine I/O register 0.
pub const BMI3_REG_FEATURE_IO1: u8 = 0x11;

/// Feature engine enable mask
pub const BMI3_FEATURE_ENGINE_ENABLE_MASK: u16 = 0x0001;

/// BMI3 I2C address
pub const BMI3_ADDR_I2C_PRIM: u8 = 0x68;
pub const BMI3_ADDR_I2C_SEC: u8 = 0x69;

pub const BMI3_ACC_ODR_MASK: u16 = 0x000F;
pub const BMI3_ACC_RANGE_MASK: u16 = 0x0070;
pub const BMI3_ACC_RANGE_POS: u8 = 4;
pub const BMI3_ACC_BW_MASK: u16 = 0x0080;
pub const BMI3_ACC_BW_POS: u8 = 7;
pub const BMI3_ACC_AVG_NUM_MASK: u16 = 0x0700;
pub const BMI3_ACC_AVG_NUM_POS: u8 = 8;
pub const BMI3_ACC_MODE_MASK: u16 = 0x7000;
pub const BMI3_ACC_MODE_POS: u8 = 12;

/// Sets the output data rate, bandwidth, range and the mode of the accelerometer
pub const BMI3_REG_ACC_CONF: u8 = 0x20;

// Sets the output data rate, bandwidth, range and the mode of the gyroscope in the sensor
pub const BMI3_REG_GYR_CONF: u8 = 0x21;

pub const BMI3_ACCEL: u8 = 0;
pub const BMI3_GYRO: u8 = 1;
pub const BMI3_SIG_MOTION: u8 = 2;
pub const BMI3_ANY_MOTION: u8 = 3;
pub const BMI3_NO_MOTION: u8 = 4;
pub const BMI3_STEP_COUNTER: u8 = 5;
pub const BMI3_TILT: u8 = 6;
pub const BMI3_ORIENTATION: u8 = 7;
pub const BMI3_FLAT: u8 = 8;
pub const BMI3_TAP: u8 = 9;
pub const BMI3_ALT_ACCEL: u8 = 10;
pub const BMI3_ALT_GYRO: u8 = 11;
pub const BMI3_ALT_AUTO_CONFIG: u8 = 12;

/// ODR in Hz
pub const BMI3_GYR_ODR_MASK: u16 = 0x000F;

/// Full scale, Resolution
pub const BMI3_GYR_RANGE_MASK: u16 = 0x0070;
pub const BMI3_GYR_RANGE_POS: u8 = 4;

/// The Gyroscope bandwidth coefficient defines the 3 dB cutoff frequency in relation to the ODR
pub const BMI3_GYR_BW_MASK: u16 = 0x0080;
pub const BMI3_GYR_BW_POS: u8 = 7;

/// Number of samples to be averaged
pub const BMI3_GYR_AVG_NUM_MASK: u16 = 0x0700;
pub const BMI3_GYR_AVG_NUM_POS: u8 = 8;

/// Defines mode of operation for Gyroscope.DO NOT COPY OPERATION DESCRIPTION TO CUSTOMER SPEC!
pub const BMI3_GYR_MODE_MASK: u16 = 0x7000;
pub const BMI3_GYR_MODE_POS: u8 = 12;

/// Mapping of feature engine interrupts to outputs
pub const BMI3_REG_INT_MAP1: u8 = 0x3A;

/// Map no-motion output to either INT1 or INT2 or IBI
pub const BMI3_NO_MOTION_OUT_MASK: u16 = 0x0003;

/// Map any-motion output to either INT1 or INT2 or IBI
pub const BMI3_ANY_MOTION_OUT_MASK: u16 = 0x000C;
pub const BMI3_ANY_MOTION_OUT_POS: u8 = 2;

/// Map flat output to either INT1 or INT2 or IBI
pub const BMI3_FLAT_OUT_MASK: u16 = 0x0030;
pub const BMI3_FLAT_OUT_POS: u8 = 4;

/// Map orientation output to either INT1 or INT2 or IBI
pub const BMI3_ORIENTATION_OUT_MASK: u8 = 0x00C0;
pub const BMI3_ORIENTATION_OUT_POS: u8 = 6;

/// Map step_detector output to either INT1 or INT2 or IBI
pub const BMI3_STEP_DETECTOR_OUT_MASK: u16 = 0x0300;
pub const BMI3_STEP_DETECTOR_OUT_POS: u8 = 8;

/// Map step_counter watermark output to either INT1 or INT2 or IBI
pub const BMI3_STEP_COUNTER_OUT_MASK: u16 = 0x0C00;
pub const BMI3_STEP_COUNTER_OUT_POS: u8 = 10;

/// Map sigmotion output to either INT1 or INT2 or IBI
pub const BMI3_SIG_MOTION_OUT_MASK: u16 = 0x3000;
pub const BMI3_SIG_MOTION_OUT_POS: u8 = 12;

/// Map tilt output to either INT1 or INT2 or IBI
pub const BMI3_TILT_OUT_MASK: u16 = 0xC000;
pub const BMI3_TILT_OUT_POS: u8 = 14;

/// Map tap output to either INT1 or INT2 or IBI
pub const BMI3_TAP_OUT_MASK: u16 = 0x0003;
pub const BMI3_TAP_OUT_POS: u8 = 0;

/// Map i3c output to either INT1 or INT2 or IBI
pub const BMI3_I3C_OUT_MASK: u16 = 0x000C;
pub const BMI3_I3C_OUT_POS: u8 = 2;

/// Map feature engine's error or status change to either INT1 or INT2 or IBI
pub const BMI3_ERR_STATUS_MASK: u16 = 0x0030;
pub const BMI3_ERR_STATUS_POS: u8 = 4;

/// Map temperature data ready interrupt to either INT1 or INT2 or IBI
pub const BMI3_TEMP_DRDY_INT_MASK: u16 = 0x00C0;
pub const BMI3_TEMP_DRDY_INT_POS: u8 = 6;

/// Map gyro data ready interrupt to either INT1 or INT2 or IBI
pub const BMI3_GYR_DRDY_INT_MASK: u16 = 0x0300;
pub const BMI3_GYR_DRDY_INT_POS: u8 = 8;

/// Map accel data ready interrupt to either INT1 or INT2 or IBI
pub const BMI3_ACC_DRDY_INT_MASK: u16 = 0x0C00;
pub const BMI3_ACC_DRDY_INT_POS: u8 = 10;

/// Map FIFO watermark interrupt to either INT1 or INT2 or IBI
pub const BMI3_FIFO_WATERMARK_INT_MASK: u16 = 0x3000;
pub const BMI3_FIFO_WATERMARK_INT_POS: u8 = 12;

/// Map FIFO full interrupt to either INT1 or INT2 or IBI
pub const BMI3_FIFO_FULL_INT_MASK: u16 = 0xC000;
pub const BMI3_FIFO_FULL_INT_POS: u8 = 14;

/******************************************************************************/
/*       Gyroscope Macro Definitions               */
/******************************************************************************/
pub const BMI3_GYR_AVG1: u8 = 0x00;
pub const BMI3_GYR_AVG2: u8 = 0x01;
pub const BMI3_GYR_AVG4: u8 = 0x02;
pub const BMI3_GYR_AVG8: u8 = 0x03;
pub const BMI3_GYR_AVG16: u8 = 0x04;
pub const BMI3_GYR_AVG32: u8 = 0x05;
pub const BMI3_GYR_AVG64: u8 = 0x06;

// Gyroscope Output Data Rate
pub const BMI3_GYR_ODR_0_78HZ: u8 = 0x01;
pub const BMI3_GYR_ODR_1_56HZ: u8 = 0x02;
pub const BMI3_GYR_ODR_3_125HZ: u8 = 0x03;
pub const BMI3_GYR_ODR_6_25HZ: u8 = 0x04;
pub const BMI3_GYR_ODR_12_5HZ: u8 = 0x05;
pub const BMI3_GYR_ODR_25HZ: u8 = 0x06;
pub const BMI3_GYR_ODR_50HZ: u8 = 0x07;
pub const BMI3_GYR_ODR_100HZ: u8 = 0x08;
pub const BMI3_GYR_ODR_200HZ: u8 = 0x09;
pub const BMI3_GYR_ODR_400HZ: u8 = 0x0A;
pub const BMI3_GYR_ODR_800HZ: u8 = 0x0B;
pub const BMI3_GYR_ODR_1600HZ: u8 = 0x0C;
pub const BMI3_GYR_ODR_3200HZ: u8 = 0x0D;
pub const BMI3_GYR_ODR_6400HZ: u8 = 0x0E;

// Gyroscope DPS Range
pub const BMI3_GYR_RANGE_125DPS: u8 = 0x00;
pub const BMI3_GYR_RANGE_250DPS: u8 = 0x01;
pub const BMI3_GYR_RANGE_500DPS: u8 = 0x02;
pub const BMI3_GYR_RANGE_1000DPS: u8 = 0x03;
pub const BMI3_GYR_RANGE_2000DPS: u8 = 0x04;

// Gyroscope mode
pub const BMI3_GYR_MODE_DISABLE: u8 = 0x00;
pub const BMI3_GYR_MODE_SUSPEND: u8 = 0x01;
pub const BMI3_GYR_MODE_LOW_PWR: u8 = 0x03;
pub const BMI3_GYR_MODE_NORMAL: u8 = 0x04;
pub const BMI3_GYR_MODE_HIGH_PERF: u8 = 0x07;

// Gyroscope bandwidth
pub const BMI3_GYR_BW_ODR_HALF: u8 = 0;
pub const BMI3_GYR_BW_ODR_QUARTER: u8 = 0;

//Bit wise to define information
pub const BMI3_I_MIN_VALUE: u8 = 1;
pub const BMI3_I_MAX_VALUE: u8 = 2;

pub const BMI3_ACC_AVG1: u8 = 0x00;
pub const BMI3_ACC_AVG2: u8 = 0x01;
pub const BMI3_ACC_AVG4: u8 = 0x02;
pub const BMI3_ACC_AVG8: u8 = 0x03;
pub const BMI3_ACC_AVG16: u8 = 0x04;
pub const BMI3_ACC_AVG32: u8 = 0x05;
pub const BMI3_ACC_AVG64: u8 = 0x06;

// Accelerometer Output Data Rate
pub const BMI3_ACC_ODR_0_78HZ: u8 = 0x01;
pub const BMI3_ACC_ODR_1_56HZ: u8 = 0x02;
pub const BMI3_ACC_ODR_3_125HZ: u8 = 0x03;
pub const BMI3_ACC_ODR_6_25HZ: u8 = 0x04;
pub const BMI3_ACC_ODR_12_5HZ: u8 = 0x05;
pub const BMI3_ACC_ODR_25HZ: u8 = 0x06;
pub const BMI3_ACC_ODR_50HZ: u8 = 0x07;
pub const BMI3_ACC_ODR_100HZ: u8 = 0x08;
pub const BMI3_ACC_ODR_200HZ: u8 = 0x09;
pub const BMI3_ACC_ODR_400HZ: u8 = 0x0A;
pub const BMI3_ACC_ODR_800HZ: u8 = 0x0B;
pub const BMI3_ACC_ODR_1600HZ: u8 = 0x0C;
pub const BMI3_ACC_ODR_3200HZ: u8 = 0x0D;
pub const BMI3_ACC_ODR_6400HZ: u8 = 0x0E;

// Accelerometer G Range
pub const BMI3_ACC_RANGE_2G: u8 = 0x00;
pub const BMI3_ACC_RANGE_4G: u8 = 0x01;
pub const BMI3_ACC_RANGE_8G: u8 = 0x02;
pub const BMI3_ACC_RANGE_16G: u8 = 0x03;

// Accelerometer mode
pub const BMI3_ACC_MODE_DISABLE: u8 = 0x00;
pub const BMI3_ACC_MODE_LOW_PWR: u8 = 0x03;
pub const BMI3_ACC_MODE_NORMAL: u8 = 0x04;
pub const BMI3_ACC_MODE_HIGH_PERF: u8 = 0x07;

// Accelerometer bandwidth
pub const BMI3_ACC_BW_ODR_HALF: u8 = 0;
pub const BMI3_ACC_BW_ODR_QUARTER: u8 = 1;

pub const BMI3_REG_INT_STATUS_INT1: u8 = 0x0D;
pub const BMI3_REG_INT_STATUS_INT2: u8 = 0x0E;

pub const BMI3_INT_STATUS_GYR_DRDY: u16 = 0x1000;
pub const BMI3_INT_STATUS_ACC_DRDY: u16 = 0x2000;

pub const BMI3_SATF_ACC_X_MASK: u16 = 0x0001;

// Saturation flag for accel Y axis
pub const BMI3_SATF_ACC_Y_MASK: u16 = 0x0002;
pub const BMI3_SATF_ACC_Y_POS: u8 = 1;

// Saturation flag for accel Z axis
pub const BMI3_SATF_ACC_Z_MASK: u16 = 0x0004;
pub const BMI3_SATF_ACC_Z_POS: u8 = 2;

// Saturation flag for gyro X axis
pub const BMI3_SATF_GYR_X_MASK: u16 = 0x0008;
pub const BMI3_SATF_GYR_X_POS: u8 = 3;

// Saturation flag for gyro Y axis
pub const BMI3_SATF_GYR_Y_MASK: u16 = 0x0010;
pub const BMI3_SATF_GYR_Y_POS: u8 = 4;

// Saturation flag for gyro Z axis
pub const BMI3_SATF_GYR_Z_MASK: u16 = 0x0020;
pub const BMI3_SATF_GYR_Z_POS: u8 = 5;

pub const BMI3_ACC_NUM_BYTES: u8 = 20;
pub const BMI3_GYR_NUM_BYTES: u8 = 14;

pub const BMI3_REG_GYR_DATA_X: u8 = 0x06;
pub const BMI3_REG_ACC_DATA_X: u8 = 0x03;

pub const BMI323_CHIP_ID: u16 = 0x0043;
pub const BMI3_16_BIT_RESOLUTION: u8 = 16;