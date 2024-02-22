use crate::bmi3_types::Bmi3Result;
use crate::bmi3dev::Bmi3Dev;
use crate::bmi3_defs::*;
use crate::enums::{Bmi3Error, Bmi3Intf};
// use core::usize;

impl Bmi3Dev {
    pub fn null_ptr_check(&self) -> Bmi3Result<()> {
        if self.read.is_none() || self.write.is_none() || self.delay_us.is_none() {
            Err(Bmi3Error::NullPtr)
        } else {
            Ok(())
        }
    }

    pub fn bmi3_soft_reset(&mut self) -> Bmi3Result<()> {
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

    pub fn bmi3_set_command_register(&mut self, command: u16) -> Bmi3Result<()> {
        let reg_data = [
            (command & BMI3_SET_LOW_BYTE) as u8,
            ((command & BMI3_SET_HIGH_BYTE) >> 8) as u8,
        ];
        return self.bmi3_set_regs(BMI3_REG_CMD, &reg_data, 2);
    }

    pub fn bmi3_set_regs(&mut self, reg_addr: u8, data: &[u8], len: u16) -> Bmi3Result<()> {
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

    pub fn bmi3_get_regs(&mut self, reg_addr: u8, data: &mut [u8], len: u16) -> Bmi3Result<()> {
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

    pub fn check_boundary_val(&mut self, val: Option<&mut u8>, min: u8, max: u8) -> Bmi3Result<()> {
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
}