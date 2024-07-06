use embassy_stm32::i2c::{Error as I2cError, I2c};
use embassy_time::Delay;
use embedded_hal_1::i2c::{I2c as EmbeddedHalI2c, ErrorType, Operation};
use embedded_hal_1::delay::DelayNs;

pub struct EmbassyI2cWrapper<'d, T: embassy_stm32::i2c::Instance, TXDMA, RXDMA>(pub I2c<'d, T, TXDMA, RXDMA>);

impl<'d, T: embassy_stm32::i2c::Instance, TXDMA, RXDMA> EmbeddedHalI2c for EmbassyI2cWrapper<'d, T, TXDMA, RXDMA> {
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.0.blocking_read(address, buffer)
    }

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.0.blocking_write(address, bytes)
    }

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.0.blocking_write_read(address, bytes, buffer)
    }

    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        for op in operations {
            match op {
                Operation::Read(buffer) => self.read(address, buffer)?,
                Operation::Write(bytes) => self.write(address, bytes)?,
            }
        }
        Ok(())
    }
}

impl<'d, T: embassy_stm32::i2c::Instance, TXDMA, RXDMA> ErrorType for EmbassyI2cWrapper<'d, T, TXDMA, RXDMA> {
    type Error = I2cError;
}

pub struct EmbassyDelayWrapper;

impl DelayNs for EmbassyDelayWrapper {
    fn delay_ns(&mut self, ns: u32) {
        Delay.delay_ns(ns);
    }

    fn delay_us(&mut self, us: u32) {
        Delay.delay_us(us);
    }

    fn delay_ms(&mut self, ms: u32) {
        Delay.delay_ms(ms);
    }
}