use crate::Error;
use embedded_hal::{i2c, spi::SpiDevice};

#[derive(Debug)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
    pub(crate) address: u8,
}

#[derive(Debug)]
pub struct SpiInterface<SPI> {
    pub(crate) spi: SPI,
}

pub trait WriteData {
    type Error;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error>;
    fn write_data(&mut self, payload: &[u8]) -> Result<(), Self::Error>;
}

impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register, data];
        self.i2c.write(self.address, &payload).map_err(Error::Comm)
    }

    fn write_data(&mut self, payload: &[u8]) -> Result<(), Self::Error> {
        self.i2c.write(self.address, payload).map_err(Error::Comm)
    }
}

impl<SPI, E> WriteData for SpiInterface<SPI>
where
    SPI: SpiDevice<Error = E>,
{
    type Error = Error<E>;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register, data];
        self.spi.write(&payload).map_err(Error::Comm)
    }

    fn write_data(&mut self, payload: &[u8]) -> Result<(), Self::Error> {
        self.spi.write(payload).map_err(Error::Comm)
    }
}

pub trait ReadData {
    type Error;
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error>;
    fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
}

impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .map_err(Error::Comm)?;
        Ok(data[0])
    }

    fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        let len = payload.len();
        self.i2c
            .write_read(self.address, &[payload[0]], &mut payload[1..len])
            .map_err(Error::Comm)
    }
}

impl<SPI, E> ReadData for SpiInterface<SPI>
where
    SPI: SpiDevice<Error = E>,
{
    type Error = Error<E>;
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut data = [register + 0x80, 0];
        self.spi.transfer_in_place(&mut data).map_err(Error::Comm)?;
        Ok(data[1])
    }

    fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        payload[0] += 0x80;
        self.spi.transfer_in_place(payload).map_err(Error::Comm)?;
        Ok(())
    }
}
