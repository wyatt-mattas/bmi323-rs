use crate::bmi3dev::Bmi3Dev;
use crate::bmi3_types::Bmi3Result;
use crate::{bmi3_defs::*, set_bits};
use crate::config::Bmi3AccelConfig;
use crate::enums::Bmi3Error;

impl Bmi3Dev{
    
    fn validate_bw_avg_acc_mode(
        &mut self,
        bandwidth: Option<&mut u8>,
        acc_mode: Option<&mut u8>,
        avg_num: Option<&mut u8>,
    ) -> Bmi3Result<()> {
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
    ) -> Bmi3Result<()> {
        if odr.is_some() && range.is_some() {
            self.check_boundary_val(odr, BMI3_ACC_ODR_0_78HZ, BMI3_ACC_ODR_6400HZ)?;
            self.check_boundary_val(range, BMI3_ACC_RANGE_2G, BMI3_ACC_RANGE_16G)
        } else {
            Err(Bmi3Error::NullPtr)
        }
    }
    
    fn accel_skipped_samples_check(&mut self, odr: f32, avg: f32) -> Bmi3Result<()> {
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
    
    fn validate_acc_odr_avg(&mut self, acc_odr: u8, acc_avg: u8) -> Bmi3Result<()> {
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
    
    pub fn set_accel_config(&mut self, config: Option<&mut Bmi3AccelConfig>) -> Bmi3Result<()> {
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
            set_bits!(reg_data, 0,
                (BMI3_ACC_ODR_MASK, config.odr as u16, None::<u8>),
                (BMI3_ACC_RANGE_MASK, config.range as u16, Some(BMI3_ACC_RANGE_POS)),
                (BMI3_ACC_BW_MASK, config.bwp as u16, Some(BMI3_ACC_BW_POS))
            );
            // avg_num and acc_mode are set in reg_data[1]
            set_bits!(reg_data, 1, 8,
                (BMI3_ACC_AVG_NUM_MASK, config.avg_num as u16, Some(BMI3_ACC_AVG_NUM_POS)),
                (BMI3_ACC_MODE_MASK, config.acc_mode as u16, Some(BMI3_ACC_MODE_POS))
            );
    
            self.bmi3_set_regs(BMI3_REG_ACC_CONF, &reg_data, 2)?;
    
            Ok(())
        } else {
            Err(Bmi3Error::NullPtr)
        }
    }
}
