use crate::Error;
use embedded_hal::{i2c, spi::SpiDevice};

/// I2C communication interface for BMI323
#[derive(Debug)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
    pub(crate) address: u8,
}

/// SPI communication interface for BMI323
#[derive(Debug)]
pub struct SpiInterface<SPI> {
    pub(crate) spi: SPI,
}

/// Trait for writing data to the BMI323
pub trait WriteData {
    type Error;
    /// Write a single byte to a register
    ///
    /// # Arguments
    ///
    /// * `register` - The register address
    /// * `data` - The byte to write
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error>;
    /// Write multiple bytes of data
    ///
    /// # Arguments
    ///
    /// * `payload` - The data to write
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
    /// Read a single byte from a register
    ///
    /// # Arguments
    ///
    /// * `register` - The register address to read from
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error>;
    /// Read multiple bytes of data
    ///
    /// # Arguments
    ///
    /// * `payload` - Buffer to store the read data
    fn read_data<'a>(&mut self, payload: &'a mut [u8]) -> Result<&'a [u8], Self::Error>;
}

impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut temp_data = [0u8; 128];
        let mut data = [0u8; 2];
        self.i2c
            .write_read(self.address, &[register], &mut temp_data)
            .map_err(Error::Comm)?;

        for i in 0..data.len() {
            data[i] = temp_data[i + 2];
        }
        Ok(data[0])
    }

    fn read_data<'a>(&mut self, payload: &'a mut [u8]) -> Result<&'a [u8], Error<E>> {
        let address = payload[0];
        let len = payload.len();
        let data = &mut payload[1..len];

        let total_len = data.len() + 2;
        let mut temp_buf = [0u8; 128]; // Temporary buffer to hold dummy bytes and data

        self.i2c
            .write_read(self.address, &[address], &mut temp_buf[..total_len])
            .map_err(Error::Comm)?;

        // Copy data from temp_buf to data, skipping dummy bytes
        data.copy_from_slice(&temp_buf[2..total_len]);

        Ok(data)
    }
}

/*
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
    }*/
