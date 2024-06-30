// use alloc::ffi::NulError;

use crate::bmi3mapint::Bmi3MapInt;
use crate::config::Bmi3SensConfig;
use crate::enums::{Bmi3Error, Bmi3Intf, Bmi3SensConfigTypes};
use crate::{bmi3_defs::*, set_bits};
// use core::usize;
use crate::bmi3_types::{Bmi3Result, I2cResult};

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
    pub read: Option<
        fn(reg_addr: u8, reg_data: &mut [u8], length: u32, intf_ptr: *mut ()) -> I2cResult<()>,
    >,
    pub write:
        Option<fn(reg_addr: u8, reg_data: &[u8], length: u32, intf_ptr: *mut ()) -> I2cResult<()>>,
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
    pub fn bmi3_init(&mut self) -> Bmi3Result<()> {
        // Early return pattern for null pointer check
        self.null_ptr_check()?; // This will return early if an Err is encountered

        self.chip_id = 0;
        self.dummy_byte = match self.intf {
            Bmi3Intf::Spi => 1,
            _ => 2,
        };

        // If any of these operations fail, the error will be propagated out of the function
        self.bmi3_soft_reset()?;
        let mut chip_id = [0u8; 2];
        self.bmi3_get_regs(BMI3_REG_CHIP_ID, &mut chip_id, 2)?;

        let mut reg_data = [0u8; 2];
        self.bmi3_get_regs(0x01, &mut reg_data, 2)?;
        let err = reg_data[0];

        self.chip_id = chip_id[0];
        self.accel_bit_width =
            if ((chip_id[1] & BMI3_REV_ID_MASK) >> BMI3_REV_ID_POS) == BMI3_ENABLE {
                BMI3_ACC_DP_OFF_XYZ_14_BIT_MASK
            } else {
                BMI3_ACC_DP_OFF_XYZ_13_BIT_MASK
            };

        // Now, handle the chip ID check within the same logical flow
        if self.chip_id as u16 == BMI323_CHIP_ID && err == 0 {
            self.resolution = BMI3_16_BIT_RESOLUTION;
            Ok(())
        } else {
            Err(Bmi3Error::DevNotFound)
        }
    }

    pub fn bmi3_get_sensor_config(&mut self, sens_cfg: &mut [Bmi3SensConfig]) -> Bmi3Result<()> {
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

    pub fn bmi3_map_interrupt(&mut self, map_int: &mut Bmi3MapInt) -> Bmi3Result<()> {
        let mut reg_data = [0u8; 4];

        // Attempt to read the current register values
        self.bmi3_get_regs(BMI3_REG_INT_MAP1, &mut reg_data, 4)?;

        set_bits!(
            reg_data,
            0,
            (
                BMI3_NO_MOTION_OUT_MASK,
                map_int.no_motion_out as u16,
                None::<u8>
            ),
            (
                BMI3_ANY_MOTION_OUT_MASK,
                map_int.any_motion_out as u16,
                Some(BMI3_ANY_MOTION_OUT_POS)
            ),
            (
                BMI3_FLAT_OUT_MASK,
                map_int.flat_out as u16,
                Some(BMI3_FLAT_OUT_POS)
            ),
            (
                BMI3_ORIENTATION_OUT_MASK as u16,
                map_int.orientation_out as u16,
                Some(BMI3_ORIENTATION_OUT_POS)
            )
        );
        set_bits!(
            reg_data,
            1,
            8,
            (
                BMI3_STEP_DETECTOR_OUT_MASK,
                map_int.step_detector_out as u16,
                Some(BMI3_STEP_DETECTOR_OUT_POS)
            ),
            (
                BMI3_STEP_COUNTER_OUT_MASK,
                map_int.step_counter_out as u16,
                Some(BMI3_STEP_COUNTER_OUT_POS)
            ),
            (
                BMI3_SIG_MOTION_OUT_MASK,
                map_int.sig_motion_out as u16,
                Some(BMI3_SIG_MOTION_OUT_POS)
            ),
            (
                BMI3_TILT_OUT_MASK,
                map_int.tilt_out as u16,
                Some(BMI3_TILT_OUT_POS)
            )
        );
        set_bits!(
            reg_data,
            2,
            (BMI3_TAP_OUT_MASK, map_int.tap_out as u16, None::<u8>),
            (
                BMI3_I3C_OUT_MASK,
                map_int.i3c_out as u16,
                Some(BMI3_I3C_OUT_POS)
            ),
            (
                BMI3_ERR_STATUS_MASK,
                map_int.err_status as u16,
                Some(BMI3_ERR_STATUS_POS)
            ),
            (
                BMI3_TEMP_DRDY_INT_MASK,
                map_int.temp_drdy_int as u16,
                Some(BMI3_TEMP_DRDY_INT_POS)
            )
        );
        set_bits!(
            reg_data,
            3,
            8,
            (
                BMI3_GYR_DRDY_INT_MASK,
                map_int.gyr_drdy_int as u16,
                Some(BMI3_GYR_DRDY_INT_POS)
            ),
            (
                BMI3_ACC_DRDY_INT_MASK,
                map_int.acc_drdy_int as u16,
                Some(BMI3_ACC_DRDY_INT_POS)
            ),
            (
                BMI3_FIFO_WATERMARK_INT_MASK,
                map_int.fifo_watermark_int as u16,
                Some(BMI3_FIFO_WATERMARK_INT_POS)
            ),
            (
                BMI3_FIFO_FULL_INT_MASK,
                map_int.fifo_full_int as u16,
                Some(BMI3_FIFO_FULL_INT_POS)
            )
        );

        // After modifying reg_data with the new interrupt settings, write back the changes
        return self.bmi3_set_regs(BMI3_REG_INT_MAP1, &mut reg_data, 4);
    }

    pub fn bmi3_set_sensor_config(&mut self, sens_cfg: &mut [Bmi3SensConfig]) -> Bmi3Result<()> {
        self.null_ptr_check()?;

        if sens_cfg.is_empty() {
            return Err(Bmi3Error::NullPtr);
        }

        for config in sens_cfg.iter_mut() {
            match &mut config.cfg {
                Some(Bmi3SensConfigTypes::Accel(accel_config)) => {
                    self.set_accel_config(Some(accel_config))?
                }
                Some(Bmi3SensConfigTypes::Gyro(gyro_config)) => {
                    // Implement gyro config setting
                    self.set_gyro_config(Some(gyro_config))?
                }
                None => (),
            };
        }

        Ok(())
    }

    pub fn get_interrupt_status(&mut self, int_status: &mut u16, register: u8) -> Bmi3Result<()> {
        let mut data_array = [0u8; 2];

        self.bmi3_get_regs(register, &mut data_array, 2).map(|()| {
            *int_status = u16::from(data_array[0]) | (u16::from(data_array[1]) << 8);
        })?;

        Ok(())
    }
}
