use crate::{bmi3_defs::*, set_bits};
use crate::bmi3mapint::Bmi3MapInt;
use crate::config::{Bmi3AccelConfig, Bmi3GyroConfig, Bmi3SensConfig, SensorConfig};
use crate::enums::{Bmi3Error, Bmi3Intf, Bmi3SensConfigTypes};
use core::usize;
use crate::bmi3_macros::*;

type Result<T> = core::result::Result<T, Bmi3Error>;

#[repr(C)]
pub struct Bmi3Dev {
    pub chip_id: u8,
    pub intf_ptr: *mut (),
    pub info: u8,
    pub intf: Bmi3Intf,
    pub intf_rslt: i8,
    pub dummy_byte: u8,
    pub resolution: u8,
    pub read_write_len: u16,
    pub read:
        Option<fn(reg_addr: u8, reg_data: &mut [u8], length: u32, intf_ptr: *mut ()) -> Result<()>>,
    pub write: Option<fn(reg_addr: u8, reg_data: &[u8], length: u32, intf_ptr: *mut ()) -> Result<()>>,
    pub delay_us: Option<fn(period: u32, intf_ptr: *mut ())>,
    pub accel_bit_width: u16,
}

impl Bmi3Dev {
    pub fn default() -> Self {
        Bmi3Dev {
            chip_id: 0,
            intf_ptr: core::ptr::null_mut(),
            info: 0,
            intf: Bmi3Intf::Spi,
            intf_rslt: BMI3_OK,
            dummy_byte: 0,
            resolution: 0,
            read_write_len: 0,
            read: None,
            write: None,
            delay_us: None,
            accel_bit_width: 0,
        }
    }
    // Assuming null_ptr_check, bmi3_soft_reset, and bmi3_get_regs are implemented elsewhere
    pub fn bmi3_init(&mut self) -> Result<()> {
        // Early return pattern for null pointer check
        self.null_ptr_check()?;

        self.chip_id = 0;
        self.dummy_byte = match self.intf {
            Bmi3Intf::Spi => 1,
            _ => 2,
        };

        self.bmi3_soft_reset()?;

        let mut chip_id = [0u8; 2];
        self.bmi3_get_regs(BMI3_REG_CHIP_ID, &mut chip_id, 2)?;

        self.chip_id = chip_id[0];
        self.accel_bit_width =
            if ((chip_id[1] & BMI3_REV_ID_MASK) >> BMI3_REV_ID_POS) == BMI3_ENABLE {
                BMI3_ACC_DP_OFF_XYZ_14_BIT_MASK
            } else {
                BMI3_ACC_DP_OFF_XYZ_13_BIT_MASK
            };

        Ok(())
    }

    fn null_ptr_check(&self) -> Result<()> {
        if self.read.is_none() || self.write.is_none() || self.delay_us.is_none() {
            Err(Bmi3Error::NullPtr)
        } else {
            Ok(())
        }
    }

    pub fn bmi3_soft_reset(&mut self) -> Result<()> {
        self.null_ptr_check()?;

        self.bmi3_set_command_register(BMI3_CMD_SOFT_RESET)?;
        self.delay_us.unwrap()(BMI3_SOFT_RESET_DELAY as u32, self.intf_ptr);

        if self.intf == Bmi3Intf::Spi {
            let mut dummy_byte = [0u8; 2];
            self.bmi3_get_regs(BMI3_REG_CHIP_ID, &mut dummy_byte, 2)?;
        }

        // Perform setup in a loop for clarity and efficiency
        let setups = [
            (BMI3_REG_FEATURE_IO2, [0x2c, 0x01]),
            (BMI3_REG_FEATURE_IO_STATUS, [BMI3_ENABLE, 0]),
            (BMI3_REG_FEATURE_CTRL, [BMI3_ENABLE, 0]),
        ];

        for (reg, data) in setups.iter() {
            self.bmi3_set_regs(*reg, data, 2)?;
        }

        // Polling loop
        let mut loops = 1;
        while loops <= 10 {
            self.delay_us.unwrap()(100000, self.intf_ptr);
            let mut reg_data = [0u8; 2];
            self.bmi3_get_regs(BMI3_REG_FEATURE_IO1, &mut reg_data, 2)?;
            if reg_data[0] & BMI3_FEATURE_ENGINE_ENABLE_MASK as u8 != 0 {
                break;
            }
            loops += 1;
        }

        Ok(())
    }

    pub fn bmi3_set_command_register(&mut self, command: u16) -> Result<()> {
        let reg_data = [
            (command & BMI3_SET_LOW_BYTE) as u8,
            ((command & BMI3_SET_HIGH_BYTE) >> 8) as u8,
        ];
        return self.bmi3_set_regs(BMI3_REG_CMD, &reg_data, 2);
    }

    pub fn bmi3_set_regs(&mut self, reg_addr: u8, data: &[u8], len: u16) -> Result<()> {
        self.null_ptr_check()?;

        if data.is_empty() {
            return Err(Bmi3Error::NullPtr);
        }

        let adjusted_reg_addr = match self.intf {
            Bmi3Intf::Spi => reg_addr | BMI3_SPI_WR_MASK as u8,
            _ => reg_addr,
        };

        match self.write {
            Some(write_fn) => match write_fn(adjusted_reg_addr, data, len as u32, self.intf_ptr) {
                Ok(()) => {
                    self.delay_us.unwrap()(2, self.intf_ptr);
                    Ok(())
                }
                _ => Err(Bmi3Error::ComFail),
            },
            None => Err(Bmi3Error::NullPtr),
        }
    }

    pub fn bmi3_get_regs(&mut self, reg_addr: u8, data: &mut [u8], len: u16) -> Result<()> {
        self.null_ptr_check()?;

        if data.is_empty() {
            return Err(Bmi3Error::NullPtr);
        }

        let adjusted_reg_addr = match self.intf {
            Bmi3Intf::Spi => reg_addr | BMI3_SPI_RD_MASK as u8,
            _ => reg_addr,
        };

        let mut temp_data = [0u8; BMI3_MAX_LEN as usize];
        match self.read {
            Some(read_fn) => match read_fn(
                adjusted_reg_addr,
                &mut temp_data,
                len as u32 + self.dummy_byte as u32,
                self.intf_ptr,
            ) {
                Ok(()) => {
                    self.delay_us.unwrap()(2, self.intf_ptr);
                    for i in 0..len as usize {
                        data[i] = temp_data[i + self.dummy_byte as usize];
                    }
                    Ok(())
                }
                _ => Err(Bmi3Error::ComFail),
            },
            None => Err(Bmi3Error::NullPtr),
        }
    }

    fn read_sensor_config(&mut self, reg_addr: u8, data_array: &mut [u8], len: u16) -> Result<()> {
        // Attempt to read the sensor configuration details
        return self.bmi3_get_regs(reg_addr, data_array, len);
    }

    // Extracts and sets sensor configuration from register data
    fn set_sensor_config<T>(
        &mut self,
        reg_data: u16,
        config: &mut T,
        sensor_masks: &[(&str, u16, u8)],
    ) where
        T: SensorConfig,
    {
        for (field, mask, pos) in sensor_masks {
            config.set_config_field(*field, self.get_bits(reg_data, *mask, *pos));
        }
    }

    // Simplified get_accel_config using the new helper function
    pub fn get_accel_config(&mut self, config: &mut Bmi3AccelConfig) -> Result<()> {
        let mut data_array = [0u8; 2];
        self.read_sensor_config(BMI3_REG_ACC_CONF, &mut data_array, 2)?;

        let reg_data = u16::from(data_array[0]);
        let sensor_masks = [
            ("odr", BMI3_ACC_ODR_MASK, 0 as u8),
            ("range", BMI3_ACC_RANGE_MASK, BMI3_ACC_RANGE_POS),
            ("bwp", BMI3_ACC_BW_MASK, BMI3_ACC_BW_POS),
            ("acc_mode", BMI3_ACC_MODE_MASK, BMI3_ACC_MODE_POS),
            ("avg_num", BMI3_ACC_AVG_NUM_MASK, BMI3_ACC_AVG_NUM_POS),
        ];
        self.set_sensor_config(reg_data, config, &sensor_masks);

        Ok(())
    }

    // Simplified get_gyro_config using the new helper function
    pub fn get_gyro_config(&mut self, config: &mut Bmi3GyroConfig) -> Result<()> {
        let mut data_array = [0u8; 2];
        self.read_sensor_config(BMI3_REG_GYR_CONF, &mut data_array, 2)?;

        let reg_data = u16::from(data_array[0]);
        let sensor_masks = [
            ("odr", BMI3_GYR_ODR_MASK, 0 as u8),
            ("range", BMI3_GYR_RANGE_MASK, BMI3_GYR_RANGE_POS),
            ("bwp", BMI3_GYR_BW_MASK, BMI3_GYR_BW_POS),
            ("gyr_mode", BMI3_GYR_MODE_MASK, BMI3_GYR_MODE_POS),
            ("avg_num", BMI3_GYR_AVG_NUM_MASK, BMI3_GYR_AVG_NUM_POS),
        ];
        self.set_sensor_config(reg_data, config, &sensor_masks);

        Ok(())
    }

    pub fn bmi3_get_sensor_config(&mut self, sens_cfg: &mut [Bmi3SensConfig]) -> Result<()> {
        self.null_ptr_check()?;

        if sens_cfg.is_empty() {
            return Err(Bmi3Error::NullPtr);
        }

        for config in sens_cfg.iter_mut() {
            let _ = match &mut config.cfg {
                Some(Bmi3SensConfigTypes::Accel(accel_config)) => {
                    self.get_accel_config(accel_config)?
                }

                Some(Bmi3SensConfigTypes::Gyro(gyro_config)) => {
                    self.get_gyro_config(gyro_config)?
                }
                None => (),
            };
        }

        Ok(())
    }

    pub fn bmi3_map_interrupt(&mut self, map_int: &mut Bmi3MapInt) -> Result<()> {
        let mut reg_data = [0u8; 4];

        // Attempt to read the current register values
        self.bmi3_get_regs(BMI3_REG_INT_MAP1, &mut reg_data, 4)?;

        // Define a helper closure to simplify bit setting
        // let set_bits = |reg_value: u16, mask: u16, value: u16, pos: Option<u8>| -> u16 {
        //     match pos {
        //         Some(shift) => (reg_value & !mask) | ((value << shift) & mask),
        //         None => (reg_value & !mask) | (value & mask),
        //     }
        // };

        set_bits!(reg_data, 0,
            (BMI3_NO_MOTION_OUT_MASK, map_int.no_motion_out as u16, None::<u8>),
            (BMI3_ANY_MOTION_OUT_MASK, map_int.any_motion_out as u16, Some(BMI3_ANY_MOTION_OUT_POS)),
            (BMI3_FLAT_OUT_MASK, map_int.flat_out as u16, Some(BMI3_FLAT_OUT_POS)),
            (BMI3_ORIENTATION_OUT_MASK as u16, map_int.orientation_out as u16, Some(BMI3_ORIENTATION_OUT_POS))
        );
        // Apply settings for each mapping, grouped by register
        // reg_data[0] = (set_bits(
        //     reg_data[0] as u16,
        //     BMI3_NO_MOTION_OUT_MASK,
        //     map_int.no_motion_out as u16,
        //     None,
        // ) | set_bits(
        //     reg_data[0] as u16,
        //     BMI3_ANY_MOTION_OUT_MASK,
        //     map_int.any_motion_out as u16,
        //     Some(BMI3_ANY_MOTION_OUT_POS),
        // ) | set_bits(
        //     reg_data[0] as u16,
        //     BMI3_FLAT_OUT_MASK,
        //     map_int.flat_out as u16,
        //     Some(BMI3_FLAT_OUT_POS),
        // ) | set_bits(
        //     reg_data[0] as u16,
        //     BMI3_ORIENTATION_OUT_MASK as u16,
        //     map_int.orientation_out as u16,
        //     Some(BMI3_ORIENTATION_OUT_POS),
        // )) as u8;

        // reg_data[1] = ((set_bits(
        //     reg_data[1] as u16,
        //     BMI3_STEP_DETECTOR_OUT_MASK,
        //     map_int.step_detector_out as u16,
        //     Some(BMI3_STEP_DETECTOR_OUT_POS),
        // ) | set_bits(
        //     reg_data[1] as u16,
        //     BMI3_STEP_COUNTER_OUT_MASK,
        //     map_int.step_counter_out as u16,
        //     Some(BMI3_STEP_COUNTER_OUT_POS),
        // ) | set_bits(
        //     reg_data[1] as u16,
        //     BMI3_SIG_MOTION_OUT_MASK,
        //     map_int.sig_motion_out as u16,
        //     Some(BMI3_SIG_MOTION_OUT_POS),
        // ) | set_bits(
        //     reg_data[1] as u16,
        //     BMI3_TILT_OUT_MASK,
        //     map_int.tilt_out as u16,
        //     Some(BMI3_TILT_OUT_POS),
        // )) >> 8) as u8;

        // reg_data[2] = (set_bits(
        //     reg_data[2] as u16,
        //     BMI3_TAP_OUT_MASK,
        //     map_int.tap_out as u16,
        //     None,
        // ) | set_bits(
        //     reg_data[2] as u16,
        //     BMI3_I3C_OUT_MASK,
        //     map_int.i3c_out as u16,
        //     Some(BMI3_I3C_OUT_POS),
        // ) | set_bits(
        //     reg_data[2] as u16,
        //     BMI3_ERR_STATUS_MASK,
        //     map_int.err_status as u16,
        //     Some(BMI3_ERR_STATUS_POS),
        // ) | set_bits(
        //     reg_data[2] as u16,
        //     BMI3_TEMP_DRDY_INT_MASK,
        //     map_int.temp_drdy_int as u16,
        //     Some(BMI3_TEMP_DRDY_INT_POS),
        // )) as u8;

        // reg_data[3] = ((set_bits(
        //     reg_data[3] as u16,
        //     BMI3_GYR_DRDY_INT_MASK,
        //     map_int.gyr_drdy_int as u16,
        //     Some(BMI3_GYR_DRDY_INT_POS),
        // ) | set_bits(
        //     reg_data[3] as u16,
        //     BMI3_ACC_DRDY_INT_MASK,
        //     map_int.acc_drdy_int as u16,
        //     Some(BMI3_ACC_DRDY_INT_POS),
        // ) | set_bits(
        //     reg_data[3] as u16,
        //     BMI3_FIFO_WATERMARK_INT_MASK,
        //     map_int.fifo_watermark_int as u16,
        //     Some(BMI3_FIFO_WATERMARK_INT_POS),
        // ) | set_bits(
        //     reg_data[3] as u16,
        //     BMI3_FIFO_FULL_INT_MASK,
        //     map_int.fifo_full_int as u16,
        //     Some(BMI3_FIFO_FULL_INT_POS),
        // )) >> 8) as u8;

        // Similarly update reg_data[1], reg_data[2], and reg_data[3] with the correct casting to u8 after combining the set_bits results

        // After modifying reg_data with the new interrupt settings, write back the changes
        return self.bmi3_set_regs(BMI3_REG_INT_MAP1, &mut reg_data, 4);
    }

    fn get_bits(&mut self, value: u16, mask: u16, pos: u8) -> u8 {
        ((value & mask) >> pos) as u8
    }

    fn set_bits(&mut self, reg_value: u16, mask: u16, value: u16, pos: Option<u8>) -> u16 {
        match pos {
            Some(shift) => (reg_value & !mask) | ((value << shift) & mask),
            None => (reg_value & !mask) | (value & mask),
        }
    }

    /// This internal API is used to validate the boundary conditions.
    fn check_boundary_val(&mut self, val: Option<&mut u8>, min: u8, max: u8) -> Result<()> {
        self.null_ptr_check()?;

        if let Some(v) = val {
            if *v < min {
                *v = min;
                self.info |= BMI3_I_MIN_VALUE; // Assuming these are predefined bit flags
            }

            if *v > max {
                *v = max;
                self.info |= BMI3_I_MAX_VALUE;
            }

            Ok(())
        } else {
            Err(Bmi3Error::NullPtr)
        }
    }

    fn validate_bw_avg_acc_mode(
        &mut self,
        bandwidth: Option<&mut u8>,
        acc_mode: Option<&mut u8>,
        avg_num: Option<&mut u8>,
    ) -> Result<()> {
        if bandwidth.is_some() && acc_mode.is_some() && avg_num.is_some() {
            self.check_boundary_val(acc_mode, BMI3_ACC_MODE_DISABLE, BMI3_ACC_MODE_HIGH_PERF)?;
            self.check_boundary_val(avg_num, BMI3_ACC_AVG1, BMI3_ACC_AVG64)?;
            self.check_boundary_val(bandwidth, BMI3_ACC_BW_ODR_HALF, BMI3_ACC_BW_ODR_QUARTER)
        } else {
            Err(Bmi3Error::NullPtr)
        }
    }

    fn validate_acc_odr_range(
        &mut self,
        odr: Option<&mut u8>,
        range: Option<&mut u8>,
    ) -> Result<()> {
        if odr.is_some() && range.is_some() {
            self.check_boundary_val(odr, BMI3_ACC_ODR_0_78HZ, BMI3_ACC_ODR_6400HZ)?;
            self.check_boundary_val(range, BMI3_ACC_RANGE_2G, BMI3_ACC_RANGE_16G)
        } else {
            Err(Bmi3Error::NullPtr)
        }
    }

    fn accel_skipped_samples_check(&mut self, odr: f32, avg: f32) -> Result<()> {
        let max_odr = 6400.0_f32;

        if odr > 0.0 && avg > 0.0 {
            let skipped_samples = max_odr / odr - avg;

            if skipped_samples > 0.0 {
                Ok(())
            } else {
                Err(Bmi3Error::AccInvalidCfg)
            }
        } else {
            Err(Bmi3Error::AccInvalidCfg)
        }
    }

    fn validate_acc_odr_avg(&mut self, acc_odr: u8, acc_avg: u8) -> Result<()> {
        let odr = match acc_odr {
            BMI3_ACC_ODR_0_78HZ => 0.78125,
            BMI3_ACC_ODR_1_56HZ => 1.5625,
            BMI3_ACC_ODR_3_125HZ => 3.125,
            BMI3_ACC_ODR_6_25HZ => 6.25,
            BMI3_ACC_ODR_12_5HZ => 12.5,
            BMI3_ACC_ODR_25HZ => 25.0,
            BMI3_ACC_ODR_50HZ => 50.0,
            BMI3_ACC_ODR_100HZ => 100.0,
            BMI3_ACC_ODR_200HZ => 200.0,
            BMI3_ACC_ODR_400HZ => 400.0,
            _ => return Err(Bmi3Error::InvalidInput), // Assuming an error variant for unsupported ODR values
        };

        let avg = match acc_avg {
            BMI3_ACC_AVG1 => 1.0,
            BMI3_ACC_AVG2 => 2.0,
            BMI3_ACC_AVG4 => 4.0,
            BMI3_ACC_AVG8 => 8.0,
            BMI3_ACC_AVG16 => 16.0,
            BMI3_ACC_AVG32 => 32.0,
            BMI3_ACC_AVG64 => 64.0,
            _ => return Err(Bmi3Error::InvalidInput), // Assuming an error variant for unsupported AVG values
        };

        self.accel_skipped_samples_check(odr, avg)
    }

    fn set_accel_config(&mut self, config: Option<&mut Bmi3AccelConfig>) -> Result<()> {
        let mut reg_data = [0u8; 2];

        if let Some(config) = config {
            self.validate_bw_avg_acc_mode(
                Some(&mut config.bwp),
                Some(&mut config.acc_mode),
                Some(&mut config.avg_num),
            )?;
            self.validate_acc_odr_range(Some(&mut config.odr), Some(&mut config.range))?;

            if config.acc_mode == BMI3_ACC_MODE_LOW_PWR {
                self.validate_acc_odr_avg(config.odr, config.avg_num)?;
            }

            if (config.acc_mode == BMI3_ACC_MODE_NORMAL
                || config.acc_mode == BMI3_ACC_MODE_HIGH_PERF)
                && (config.odr >= BMI3_ACC_ODR_0_78HZ && config.odr <= BMI3_ACC_ODR_6_25HZ)
            {
                return Err(Bmi3Error::AccInvalidCfg);
            }

            // ODR, range, and BWP are set in reg_data[0]
            reg_data[0] = (self.set_bits(
                reg_data[0] as u16,
                BMI3_ACC_ODR_MASK,
                config.odr as u16,
                None,
            ) | self.set_bits(
                reg_data[0] as u16,
                BMI3_ACC_RANGE_MASK,
                config.avg_num as u16,
                Some(BMI3_ACC_RANGE_POS),
            ) | self.set_bits(
                reg_data[0] as u16,
                BMI3_ACC_BW_MASK,
                config.bwp as u16,
                Some(BMI3_ACC_BW_POS),
            )) as u8;
            // avg_num and acc_mode are set in reg_data[1]
            reg_data[1] = (self.set_bits(
                reg_data[1] as u16,
                BMI3_ACC_AVG_NUM_MASK,
                config.avg_num as u16,
                Some(BMI3_ACC_AVG_NUM_POS),
            ) | self.set_bits(
                reg_data[1] as u16,
                BMI3_ACC_MODE_MASK,
                config.acc_mode as u16,
                Some(BMI3_ACC_MODE_POS),
            ) >> 8) as u8;

            // TODO finish changing to using result type instead of i8
            self.bmi3_set_regs(BMI3_REG_ACC_CONF, &reg_data, 2)?;

            Ok(())
        } else {
            Err(Bmi3Error::NullPtr)
        }
    }
}
