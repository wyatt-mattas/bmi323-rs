// device.rs
use crate::{
    interface::{I2cInterface, ReadData, SpiInterface, WriteData},
    sensor_data::*,
    types::{AccelerometerRange, GyroscopeRange},
    AccelerometerConfig, Bmi323, Error, GyroscopeConfig, Register,
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
    pub fn new(iface: DI, delay: D) -> Self {
        Bmi323 {
            iface,
            delay,
            accel_range: AccelerometerRange::default(),
            gyro_range: GyroscopeRange::default(),
        }
    }

    pub fn init(&mut self) -> Result<(), Error<E>> {
        // Perform soft reset
        self.write_register_16bit(Register::CMD, Register::CMD_SOFT_RESET)?;
        self.delay_ms(1);

        // Check chip ID
        let chip_id = self.read_register(Register::CHIPID)?;
        if chip_id != Register::BMI323_CHIP_ID {
            return Err(Error::InvalidDevice);
        }

        // Configure default settings
        self.set_accel_config(AccelerometerConfig::default())?;
        self.set_gyro_config(GyroscopeConfig::default())?;

        Ok(())
    }

    pub fn set_accel_config(&mut self, config: AccelerometerConfig) -> Result<(), Error<E>> {
        let mut reg_data = [0u8; 2];
        reg_data[0] = config.odr | (config.range.bits() << 4) | (config.bw << 7);
        reg_data[1] = config.avg_num | (config.mode << 4);
        self.write_register(Register::ACC_CONF, reg_data[0])?;
        self.write_register(Register::ACC_CONF + 1, reg_data[1])?;
        self.accel_range = config.range;
        Ok(())
    }

    pub fn set_gyro_config(&mut self, config: GyroscopeConfig) -> Result<(), Error<E>> {
        let mut reg_data = [0u8; 2];
        reg_data[0] = config.odr | (config.range.bits() << 4) | (config.bw << 7);
        reg_data[1] = config.avg_num | (config.mode << 4);
        self.write_register(Register::GYR_CONF, reg_data[0])?;
        self.write_register(Register::GYR_CONF + 1, reg_data[1])?;
        self.gyro_range = config.range;
        Ok(())
    }

    pub fn read_accel_data(&mut self) -> Result<AccelerometerData<16>, Error<E>> {
        let mut data = [0u8; 6];
        self.read_data(&mut data)?;
        Ok(AccelerometerData {
            x: i16::from_le_bytes([data[0], data[1]]),
            y: i16::from_le_bytes([data[2], data[3]]),
            z: i16::from_le_bytes([data[4], data[5]]),
        })
    }

    pub fn read_gyro_data(&mut self) -> Result<GyroscopeData<16>, Error<E>> {
        let mut data = [0u8; 6];
        self.read_data(&mut data)?;
        Ok(GyroscopeData {
            x: i16::from_le_bytes([data[0], data[1]]),
            y: i16::from_le_bytes([data[2], data[3]]),
            z: i16::from_le_bytes([data[4], data[5]]),
        })
    }

    pub fn read_accel_data_scaled(&mut self) -> Result<AccelerometerDataScaled, Error<E>> {
        let raw_data = self.read_accel_data()?;
        let g = self.accel_range.to_g();
        Ok(raw_data.to_mps2(g))
    }

    pub fn read_gyro_data_scaled(&mut self) -> Result<GyroscopeDataScaled, Error<E>> {
        let raw_data = self.read_gyro_data()?;
        let dps = self.gyro_range.to_dps();
        Ok(raw_data.to_dps(dps))
    }

    fn write_register(&mut self, reg: u8, value: u8) -> Result<(), Error<E>> {
        self.iface.write_register(reg, value)
    }

    fn write_register_16bit(&mut self, reg: u8, value: u16) -> Result<(), Error<E>> {
        let bytes = value.to_le_bytes();
        self.iface.write_data(&[reg, bytes[0], bytes[1]])
    }

    fn read_register(&mut self, reg: u8) -> Result<u8, Error<E>> {
        self.iface.read_register(reg)
    }

    fn read_data(&mut self, data: &mut [u8]) -> Result<(), Error<E>> {
        self.iface.read_data(data)
    }

    fn delay_ms(&mut self, ms: u32) {
        self.delay.delay_ms(ms);
    }
}
