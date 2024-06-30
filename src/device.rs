use crate::{
    interface::{I2cInterface, ReadData, SpiInterface, WriteData},
    types::{AccelerometerRange, GyroscopeRange, Sensor3DData, Sensor3DDataScaled, SensorType},
    AccelConfig, Bmi323, Error, GyroConfig, Register,
};
use embedded_hal::delay::DelayNs;

impl<I2C, D> Bmi323<I2cInterface<I2C>, D>
where
    D: DelayNs,
{
    pub fn new_with_i2c(i2c: I2C, address: u8, delay: D) -> Self {
        Bmi323 {
            iface: I2cInterface { i2c, address },
            delay,
            accel_range: AccelerometerRange::default(),
            gyro_range: GyroscopeRange::default(),
        }
    }
}

impl<SPI, D> Bmi323<SpiInterface<SPI>, D>
where
    D: DelayNs,
{
    pub fn new_with_spi(spi: SPI, delay: D) -> Self {
        Bmi323 {
            iface: SpiInterface { spi },
            delay,
            accel_range: AccelerometerRange::default(),
            gyro_range: GyroscopeRange::default(),
        }
    }
}

impl<DI, D, E> Bmi323<DI, D>
where
    DI: ReadData<Error = Error<E>> + WriteData<Error = Error<E>>,
    D: DelayNs,
{
    /// Create a new BMI323 device instance
    ///
    /// # Arguments
    ///
    /// * `iface` - The communication interface (I2C or SPI)
    /// * `delay` - A delay provider
    pub fn new(iface: DI, delay: D) -> Self {
        Self {
            iface,
            delay,
            accel_range: AccelerometerRange::default(),
            gyro_range: GyroscopeRange::default(),
        }
    }

    /// Initialize the device
    pub fn init(&mut self) -> Result<(), Error<E>> {
        self.set_command_register(Register::CMD_SOFT_RESET)?;
        //self.write_register_16bit(Register::CMD, Register::CMD_SOFT_RESET)?;
        self.delay.delay_us(2000);

        let mut reg_data = [0u8; 2];
        reg_data[0] = 0x01; // sensor error conditins register
        self.read_data(&mut reg_data)?;
        if reg_data[0] != Register::BMI323_CHIP_ID {
            return Err(Error::InvalidDevice);
        }

        let mut reg_data = [0u8; 2];
        reg_data[0] = Register::CHIPID;
        self.read_data(&mut reg_data)?;
        if reg_data[0] != Register::BMI323_CHIP_ID {
            return Err(Error::InvalidDevice);
        }

        Ok(())
    }

    /// Soft reset the device
    /// Perform a soft reset of the BMI323 device
    /*
    fn soft_reset(&mut self) -> Result<(), Error<E>> {
        self.write_register_16bit(Register::CMD, Register::CMD_SOFT_RESET)?;
        self.delay.delay_ms(2);

        // Perform setup
        let setups = [
            (Register::FEATURE_IO2, [0x2c, 0x01]),
            (Register::FEATURE_IO_STATUS, [0x01, 0]),
            (Register::FEATURE_CTRL, [0x01, 0]),
        ];

        for (reg, data) in setups.iter() {
            self.write_register_16bit(*reg, u16::from_le_bytes(*data))?;
        }

        // Polling loop
        for _ in 0..10 {
            self.delay.delay_ms(100);
            let mut reg_data = [0u8; 2];
            reg_data[0] = Register::FEATURE_IO1;
            self.read_data(&mut reg_data)?;
            if reg_data[0] == 1 {
                break;
            }
        }

        Ok(())
        } */

    /// Set a command in the command register
    fn set_command_register(&mut self, command: u16) -> Result<(), Error<E>> {
        const BMI3_SET_LOW_BYTE: u16 = 0x00FF;
        const BMI3_SET_HIGH_BYTE: u16 = 0xFF00;

        let reg_data = [
            (command & BMI3_SET_LOW_BYTE) as u8,
            ((command & BMI3_SET_HIGH_BYTE) >> 8) as u8,
        ];
        self.write_register_16bit(Register::CMD, u16::from_le_bytes(reg_data))
    }

    /// Set the accelerometer configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The accelerometer configuration
    pub fn set_accel_config(&mut self, config: AccelConfig) -> Result<(), Error<E>> {
        let reg_data = self.config_to_reg_data(config);
        self.write_register_16bit(Register::ACC_CONF, reg_data)?;
        self.accel_range = config.range;
        Ok(())
    }

    /// Set the gyroscope configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The gyroscope configuration
    pub fn set_gyro_config(&mut self, config: GyroConfig) -> Result<(), Error<E>> {
        let reg_data = self.config_to_reg_data(config);
        self.write_register_16bit(Register::GYR_CONF, reg_data)?;
        self.gyro_range = config.range;
        Ok(())
    }

    fn config_to_reg_data<T>(&self, config: T) -> u16
    where
        T: Into<u16> + Copy,
    {
        let config: u16 = config.into();
        config
    }

    fn read_sensor_data(&mut self, sensor_type: SensorType) -> Result<Sensor3DData, Error<E>> {
        let (base_reg, data_size) = match sensor_type {
            SensorType::Accelerometer => (Register::ACC_DATA_X, 21),
            SensorType::Gyroscope => (Register::GYR_DATA_X, 15),
        };

        let mut data = [0u8; 21]; // Use the larger size
        data[0] = base_reg;
        self.read_data(&mut data[0..data_size])?;

        Ok(Sensor3DData {
            x: i16::from_le_bytes([data[1], data[2]]),
            y: i16::from_le_bytes([data[3], data[4]]),
            z: i16::from_le_bytes([data[5], data[6]]),
        })
    }

    pub fn read_accel_data(&mut self) -> Result<Sensor3DData, Error<E>> {
        self.read_sensor_data(SensorType::Accelerometer)
    }

    pub fn read_gyro_data(&mut self) -> Result<Sensor3DData, Error<E>> {
        self.read_sensor_data(SensorType::Gyroscope)
    }

    pub fn read_accel_data_scaled(&mut self) -> Result<Sensor3DDataScaled, Error<E>> {
        let raw_data = self.read_accel_data()?;
        Ok(raw_data.to_mps2(self.accel_range.to_g())) // Assuming 16-bit width
    }

    pub fn read_gyro_data_scaled(&mut self) -> Result<Sensor3DDataScaled, Error<E>> {
        let raw_data = self.read_gyro_data()?;
        Ok(raw_data.to_dps(self.gyro_range.to_dps())) // Assuming 16-bit width
    }

    /*
    pub fn read_temperature(&mut self) -> Result<f32, Error<E>> {
        let mut data = [0u8; 2];
        self.read_data(&mut data)?;
        let raw_temp = i16::from_le_bytes([data[0], data[1]]);
        Ok((raw_temp as f32 / 512.0) + 23.0)
    }

    pub fn read_sensor_time(&mut self) -> Result<u32, Error<E>> {
        let mut data = [0u8; 3];
        self.read_data(&mut data)?;
        Ok(u32::from_le_bytes([data[0], data[1], data[2], 0]))
        }

    fn write_register(&mut self, reg: u8, value: u8) -> Result<(), Error<E>> {
        self.iface.write_register(reg, value)
        }*/

    fn write_register_16bit(&mut self, reg: u8, value: u16) -> Result<(), Error<E>> {
        let bytes = value.to_le_bytes();
        self.iface.write_data(&[reg, bytes[0], bytes[1]])
    }

    /*
    fn read_register(&mut self, reg: u8) -> Result<u8, Error<E>> {
        self.iface.read_register(reg)
        }*/

    fn read_data(&mut self, data: &mut [u8]) -> Result<(), Error<E>> {
        self.iface.read_data(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sensor3d_data(data: &[u8]) -> Sensor3DData {
        Sensor3DData {
            x: i16::from_le_bytes([data[0], data[1]]),
            y: i16::from_le_bytes([data[2], data[3]]),
            z: i16::from_le_bytes([data[4], data[5]]),
        }
    }

    mod sensor3d_data {
        use super::*;

        #[test]
        fn can_decode_positive_array() {
            let result = get_sensor3d_data(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
            assert_eq!(
                result,
                Sensor3DData {
                    x: 0x0201,
                    y: 0x0403,
                    z: 0x0605
                }
            );
        }

        #[test]
        fn can_decode_negative_array() {
            let result = get_sensor3d_data(&[0x0B, 0x86, 0x0B, 0x86, 0x0B, 0x86]);
            assert_eq!(
                result,
                Sensor3DData {
                    x: -31221,
                    y: -31221,
                    z: -31221
                }
            );
        }
    }
}
