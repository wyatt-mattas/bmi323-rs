// device.rs

use crate::{
    interface::{I2cInterface, ReadData, SpiInterface, WriteData},
    types::{AccelerometerRange, GyroscopeRange},
    Bmi323, Error, Register,
};

impl<I2C> Bmi323<I2cInterface<I2C>> {
    pub fn new_with_i2c(i2c: I2C, address: u8) -> Self {
        Bmi323 {
            iface: I2cInterface { i2c, address },
            accel_range: AccelerometerRange::default(),
            gyro_range: GyroscopeRange::default(),
        }
    }
}

impl<SPI> Bmi323<SpiInterface<SPI>> {
    pub fn new_with_spi(spi: SPI) -> Self {
        Bmi323 {
            iface: SpiInterface { spi },
            accel_range: AccelerometerRange::default(),
            gyro_range: GyroscopeRange::default(),
        }
    }
}

impl<DI, E> Bmi323<DI>
where
    DI: ReadData<Error = Error<E>> + WriteData<Error = Error<E>>,
{
    pub fn initialize(&mut self) -> Result<(), Error<E>> {
        // Add initialization logic here
        self.set_accel_power_mode(0x11)?; // Example power mode setting
        self.set_gyro_power_mode(0x15)?; // Example power mode setting
        Ok(())
    }

    pub fn set_accel_power_mode(&mut self, mode: u8) -> Result<(), Error<E>> {
        self.iface
            .write_register(Register::ACCEL_PWR_CTRL as u8, mode)
    }

    pub fn set_gyro_power_mode(&mut self, mode: u8) -> Result<(), Error<E>> {
        self.iface
            .write_register(Register::GYRO_PWR_CTRL as u8, mode)
    }

    pub fn read_chip_id(&mut self) -> Result<u8, Error<E>> {
        self.iface.read_register(Register::CHIPID as u8)
    }

    pub fn read_status(&mut self) -> Result<u8, Error<E>> {
        self.iface.read_register(Register::STATUS as u8)
    }

    pub fn read_sensor_data(&mut self) -> Result<[u8; 6], Error<E>> {
        let mut data = [0; 6];
        self.iface.read_data(&mut data)?;
        Ok(data)
    }

    pub fn configure_sensor(&mut self) -> Result<(), Error<E>> {
        // Add configuration logic here
        Ok(())
    }
}
