use crate::{bmi3_defs::*, bmi3dev::Bmi3Dev, enums::Bmi3Error};

#[derive(Debug, Default, Copy, Clone)]
pub struct Bmi3SensAxesData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub sens_time: u32,
    pub sat_x: u8,
    pub sat_y: u8,
    pub sat_z: u8,
}

impl Bmi3SensAxesData {
    pub fn default() -> Self {
        Bmi3SensAxesData{
            x: 0,
            y: 0,
            z: 0,
            sens_time: 0,
            sat_x: 1,
            sat_y: 1,
            sat_z: 1,
        }
    }

    fn from_acc_reg_data(reg_data: &[u16]) -> Self {
        Self {
            x: reg_data[0] as i16,
            y: reg_data[1] as i16,
            z: reg_data[2] as i16,
            sens_time: reg_data[3] as u32 | ((reg_data[4] as u32) << 16),
            sat_x: reg_data[5] as u8 & BMI3_SATF_ACC_X_MASK as u8,
            sat_y: (reg_data[5] as u8 & BMI3_SATF_ACC_Y_MASK as u8) >> BMI3_SATF_ACC_Y_POS,
            sat_z: (reg_data[5] as u8 & BMI3_SATF_ACC_Z_MASK as u8) >> BMI3_SATF_ACC_Z_POS,
        }
    }

    fn from_gyro_reg_data(reg_data: &[u16]) -> Self {
        // Assuming get_gyr_data is similar to get_acc_data but tailored for gyroscope data
        // This function would similarly translate gyro data from register values
        // Implementation would be analogous to from_acc_reg_data, adjusted for gyro specifics
        Self {
            x: reg_data[0] as i16,
            y: reg_data[1] as i16,
            z: reg_data[2] as i16,
            sens_time: reg_data[3] as u32 | ((reg_data[4] as u32) << 16),
            sat_x: reg_data[5] as u8 & BMI3_SATF_GYR_X_MASK as u8,
            sat_y: (reg_data[5] as u8 & BMI3_SATF_GYR_Y_MASK as u8) >> BMI3_SATF_GYR_Y_POS,
            sat_z: (reg_data[5] as u8 & BMI3_SATF_GYR_Z_MASK as u8) >> BMI3_SATF_GYR_Z_POS,
        }
    }
}

impl Bmi3Dev {
    // Assuming Bmi3Dev and bmi3_get_regs are defined elsewhere

    pub fn get_accel_sensor_data(&mut self, reg_addr: u8) -> Result<Bmi3SensAxesData, Bmi3Error> {
        let mut reg_data = [0u8; BMI3_ACC_NUM_BYTES as usize]; // Define BMI3_ACC_NUM_BYTES accordingly

        // Simulate reading from the device
        // self.bmi3_get_regs(reg_addr, &mut reg_data, BMI3_ACC_NUM_BYTES as u16);

        match self.bmi3_get_regs(reg_addr, &mut reg_data, BMI3_ACC_NUM_BYTES as u16) {
            Ok(_) => {
                let acc_data = [
                    (reg_data[0] as u16) | ((reg_data[1] as u16) << 8),
                    (reg_data[2] as u16) | ((reg_data[3] as u16) << 8),
                    (reg_data[4] as u16) | ((reg_data[5] as u16) << 8),
                    (reg_data[14] as u16) | ((reg_data[15] as u16) << 8),
                    (reg_data[16] as u16) | ((reg_data[17] as u16) << 8),
                    reg_data[18] as u16,
                ];
                Ok(Bmi3SensAxesData::from_acc_reg_data(&acc_data))
            },
            Err(_) => Err(Bmi3Error::NullPtr)
        }
    }

    pub fn get_gyro_sensor_data(&mut self, reg_addr: u8) -> Result<Bmi3SensAxesData, Bmi3Error> {
        let mut reg_data = [0u8; BMI3_GYR_NUM_BYTES as usize]; // Define BMI3_GYR_NUM_BYTES accordingly

        // Simulate reading from the device
        // self.bmi3_get_regs(reg_addr, &mut reg_data, BMI3_GYR_NUM_BYTES as u16);

        match self.bmi3_get_regs(reg_addr, &mut reg_data, BMI3_GYR_NUM_BYTES as u16) {
            Ok(_) => {
                let gyr_data = [
                    (reg_data[0] as u16) | ((reg_data[1] as u16) << 8),
                    (reg_data[2] as u16) | ((reg_data[3] as u16) << 8),
                    (reg_data[4] as u16) | ((reg_data[5] as u16) << 8),
                    (reg_data[8] as u16) | ((reg_data[9] as u16) << 8),
                    (reg_data[10] as u16) | ((reg_data[11] as u16) << 8),
                    reg_data[12] as u16,
                ];
                Ok(Bmi3SensAxesData::from_gyro_reg_data(&gyr_data))
            },
            Err(_) => Err(Bmi3Error::NullPtr),
        }
    }
}
