use crate::bmi3dev::Bmi3Dev;
use crate::bmi3_types::Bmi3Result;
use crate::bmi3_defs::*;
use crate::config::{Bmi3AccelConfig, Bmi3GyroConfig, SensorConfig};

impl Bmi3Dev {

    fn get_bits(&mut self, value: u16, mask: u16, pos: u8) -> u8 {
        ((value & mask) >> pos) as u8
    }

    fn read_sensor_config(&mut self, reg_addr: u8, data_array: &mut [u8], len: u16) -> Bmi3Result<()> {
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
    pub fn get_accel_config(&mut self, config: &mut Bmi3AccelConfig) -> Bmi3Result<()> {
        let mut data_array = [0u8; 2];
        self.read_sensor_config(BMI3_REG_ACC_CONF, &mut data_array, 2)?;

        // Combine data_array[0] and data_array[1] into a single u16 with proper shifting
        let reg_data_low = u16::from(data_array[0]); // Low byte (no shift needed)
        let reg_data_high = u16::from(data_array[1]) << 8; // Shift high byte left by 8 bits
        let reg_data = reg_data_high | reg_data_low; // Combine high and low parts
    
        // let reg_data = u16::from(data_array[0]);
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
    pub fn get_gyro_config(&mut self, config: &mut Bmi3GyroConfig) -> Bmi3Result<()> {
        let mut data_array = [0u8; 2];
        self.read_sensor_config(BMI3_REG_GYR_CONF, &mut data_array, 2)?;

        // Combine data_array[0] and data_array[1] into a single u16 with proper shifting
        let reg_data_low = u16::from(data_array[0]); // Low byte (no shift needed)
        let reg_data_high = u16::from(data_array[1]) << 8; // Shift high byte left by 8 bits
        let reg_data = reg_data_high | reg_data_low; // Combine high and low parts
    
        // let reg_data = u16::from(data_array[0]);
        let sensor_masks = [
            ("odr", BMI3_GYR_ODR_MASK, 0 as u8),
            ("range", BMI3_GYR_RANGE_MASK, BMI3_GYR_RANGE_POS),
            ("bwp", BMI3_GYR_BW_MASK, BMI3_GYR_BW_POS),
            ("gyr_mode", BMI3_GYR_MODE_MASK, BMI3_GYR_MODE_POS),
            ("avg_num", BMI3_GYR_AVG_NUM_MASK, BMI3_GYR_AVG_NUM_POS),
        ];
        self.set_sensor_config(reg_data, config, &sensor_masks);

        // let sensor_masks_high = [
        //     ("gyr_mode", BMI3_GYR_MODE_MASK, BMI3_GYR_MODE_POS),
        //     ("avg_num", BMI3_GYR_AVG_NUM_MASK, BMI3_GYR_AVG_NUM_POS),
        // ];
        // self.set_sensor_config(reg_data_high, config, &sensor_masks_high);
        Ok(())
    }
}
