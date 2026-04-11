use crate::Error;
use embedded_hal_async::{i2c, spi::SpiDevice};

/// I2C communication interface for BMI323
#[derive(Debug)]
pub struct AsyncI2cInterface<I2C> {
    pub(crate) i2c: I2C,
    pub(crate) address: u8,
}

/// SPI communication interface for BMI323
#[derive(Debug)]
pub struct AsyncSpiInterface<SPI> {
    pub(crate) spi: SPI,
}

/// Trait for writing data to the BMI323
#[allow(async_fn_in_trait)]
pub trait AsyncWriteData {
    type Error;

    /// Write multiple bytes of data
    async fn write_data(&mut self, payload: &[u8]) -> Result<(), Self::Error>;
}

/// Trait for reading data from the BMI323
#[allow(async_fn_in_trait)]
pub trait AsyncReadData {
    type Error;

    /// Read a single byte from a register
    async fn read_register(&mut self, register: u8) -> Result<u8, Self::Error>;

    /// Read multiple bytes of data
    async fn read_data<'a>(&mut self, payload: &'a mut [u8]) -> Result<&'a [u8], Self::Error>;
}

impl<I2C, E> AsyncWriteData for AsyncI2cInterface<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;

    async fn write_data(&mut self, payload: &[u8]) -> Result<(), Self::Error> {
        self.i2c.write(self.address, payload).await.map_err(Error::Comm)
    }
}

impl<I2C, E> AsyncReadData for AsyncI2cInterface<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;

    async fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut temp_data = [0u8; 128];
        let mut data = [0u8; 2];
        self.i2c
            .write_read(self.address, &[register], &mut temp_data)
            .await
            .map_err(Error::Comm)?;
        for i in 0..data.len() {
            data[i] = temp_data[i + 2];
        }
        Ok(data[0])
    }

    async fn read_data<'a>(&mut self, payload: &'a mut [u8]) -> Result<&'a [u8], Self::Error> {
        let address = payload[0];
        let len = payload.len();
        let data = &mut payload[1..len];

        let total_len = data.len() + 2;
        let mut temp_buf = [0u8; 128]; // Temporary buffer to hold dummy bytes and data

        self.i2c
            .write_read(self.address, &[address], &mut temp_buf[..total_len])
            .await
            .map_err(Error::Comm)?;

        // Copy data from temp_buf to data, skipping dummy bytes
        data.copy_from_slice(&temp_buf[2..total_len]);

        Ok(data)
    }
}

impl<SPI, E> AsyncWriteData for AsyncSpiInterface<SPI>
where
    SPI: SpiDevice<Error = E>,
{
    type Error = Error<E>;

    async fn write_data(&mut self, payload: &[u8]) -> Result<(), Self::Error> {
        self.spi.write(payload).await.map_err(Error::Comm)
    }
}

impl<SPI, E> AsyncReadData for AsyncSpiInterface<SPI>
where
    SPI: SpiDevice<Error = E>,
{
    type Error = Error<E>;

    async fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut data = [register | 0x80, 0, 0]; // Add read bit and 1 dummy byte
        self.spi.transfer_in_place(&mut data).await.map_err(Error::Comm)?;
        Ok(data[2]) // Return the actual data byte, skipping dummy byte
    }

    async fn read_data<'a>(&mut self, payload: &'a mut [u8]) -> Result<&'a [u8], Self::Error> {
        let len = payload.len();
        let mut temp_buf = [0u8; 128]; // Temporary buffer to hold read bit, dummy byte, and data
        temp_buf[0] = payload[0] | 0x80; // Add read bit to register address

        self.spi
            .transfer_in_place(&mut temp_buf[..len + 1])
            .await
            .map_err(Error::Comm)?; // +1 for dummy byte

        // Copy data from temp_buf to payload, skipping dummy byte
        payload[1..].copy_from_slice(&temp_buf[2..len + 1]);
        Ok(&payload[1..])
    }
}