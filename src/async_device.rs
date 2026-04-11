use crate::{
    async_interface::{AsyncI2cInterface, AsyncReadData, AsyncSpiInterface, AsyncWriteData},
    types::{AccelerometerRange, GyroscopeRange, Sensor3DData, Sensor3DDataScaled, SensorType},
    AccelConfig, AsyncBmi323, Error, GyroConfig, Register,
};
use embedded_hal_async::delay::DelayNs;

impl<I2C, D> AsyncBmi323<AsyncI2cInterface<I2C>, D>
where
    D: DelayNs,
{
    /// Create a new BMI323 device instance
    ///
    /// # Arguments
    ///
    /// * `iface` - The communication interface
    /// * `delay` - A delay provider
    pub fn new_with_i2c(i2c: I2C, address: u8, delay: D) -> Self {
        AsyncBmi323 {
            iface: AsyncI2cInterface { i2c, address },
            delay,
            accel_range: AccelerometerRange::default(),
            gyro_range: GyroscopeRange::default(),
        }
    }
}

impl<SPI, D> AsyncBmi323<AsyncSpiInterface<SPI>, D>
where
    D: DelayNs,
{
    /// Create a new BMI323 device instance
    ///
    /// # Arguments
    ///
    /// * `iface` - The communication interface
    /// * `delay` - A delay provider
    pub fn new_with_spi(spi: SPI, delay: D) -> Self {
        AsyncBmi323 {
            iface: AsyncSpiInterface { spi },
            delay,
            accel_range: AccelerometerRange::default(),
            gyro_range: GyroscopeRange::default(),
        }
    }
}

impl<DI, D, E> AsyncBmi323<DI, D>
where
    DI: AsyncReadData<Error = Error<E>> + AsyncWriteData<Error = Error<E>>,
    D: DelayNs,
{
    /// Initialize the device
    pub async fn init(&mut self) -> Result<(), Error<E>> {
        self.write_register_16bit(Register::CMD, Register::CMD_SOFT_RESET)
            .await?;

        self.delay.delay_us(2000).await;

        //let mut reg_data = [0u8; 3];
        //reg_data[0] = 0x01; // sensor error conditins register
        let status = self.read_register(0x01).await?;
        if (status & 0b0000_0001) != 0 {
            return Err(Error::InvalidDevice);
        }

        let result = self.read_register(Register::CHIPID).await?;
        if result != Register::BMI323_CHIP_ID {
            return Err(Error::InvalidDevice);

        }

        Ok(())
    }

    /// Set the accelerometer configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The accelerometer configuration
    pub async fn set_accel_config(&mut self, config: AccelConfig) -> Result<(), Error<E>> {
        let reg_data = self.config_to_reg_data(config);
        self.write_register_16bit(Register::ACC_CONF, reg_data).await?;
        self.accel_range = config.range;

        // Wait for accelerometer data to be ready
        self.wait_for_data_ready(SensorType::Accelerometer).await?;

        Ok(())
    }

    /// Set the gyroscope configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The gyroscope configuration
    pub async fn set_gyro_config(&mut self, config: GyroConfig) -> Result<(), Error<E>> {
        let reg_data = self.config_to_reg_data(config);
        self.write_register_16bit(Register::GYR_CONF, reg_data).await?;
        self.gyro_range = config.range;

        // Wait for gyroscope data to be ready
        self.wait_for_data_ready(SensorType::Gyroscope).await?;

        Ok(())
    }

    fn config_to_reg_data<T>(&self, config: T) -> u16
    where
        T: Into<u16> + Copy,
    {
        let config: u16 = config.into();
        config
    }

    async fn read_sensor_data(&mut self, sensor_type: SensorType) -> Result<Sensor3DData, Error<E>> {
        let (base_reg, data_size) = match sensor_type {
            SensorType::Accelerometer => (Register::ACC_DATA_X, 21),
            SensorType::Gyroscope => (Register::GYR_DATA_X, 15),
        };

        let mut data = [0u8; 21]; // Use the larger size
        data[0] = base_reg;
        let sensor_data = self.read_data(&mut data[0..data_size]).await?;

        Ok(Sensor3DData {
            x: i16::from_le_bytes([sensor_data[0], sensor_data[1]]),
            y: i16::from_le_bytes([sensor_data[2], sensor_data[3]]),
            z: i16::from_le_bytes([sensor_data[4], sensor_data[5]]),
        })
    }

    /// Read the LSB for the accelerometer
    pub async fn read_accel_data(&mut self) -> Result<Sensor3DData, Error<E>> {
        self.read_sensor_data(SensorType::Accelerometer).await
    }

    /// Read the LSB for the gyroscope
    pub async fn read_gyro_data(&mut self) -> Result<Sensor3DData, Error<E>> {
        self.read_sensor_data(SensorType::Gyroscope).await
    }

    /// Read the LSB for the accelerometer and return the scaled value as mps2
    pub async fn read_accel_data_scaled(&mut self) -> Result<Sensor3DDataScaled, Error<E>> {
        self.read_accel_data()
            .await
            .map(|raw_data| raw_data.to_mps2(self.accel_range.to_g()))
    }

    /// Read the LSB for the gyroscope and return the scaled value as dps
    pub async fn read_gyro_data_scaled(&mut self) -> Result<Sensor3DDataScaled, Error<E>> {
        self.read_gyro_data()
            .await
            .map(|raw_data| raw_data.to_dps(self.gyro_range.to_dps()))
    }

    async fn write_register_16bit(&mut self, reg: u8, value: u16) -> Result<(), Error<E>> {
        let bytes = value.to_le_bytes();
        self.iface.write_data(&[reg, bytes[0], bytes[1]]).await
    }

    async fn read_register(&mut self, reg: u8) -> Result<u8, Error<E>> {
        self.iface.read_register(reg).await
    }

    async fn read_data<'a>(&mut self, data: &'a mut [u8]) -> Result<&'a [u8], Error<E>> {
        self.iface.read_data(data).await
    }

    async fn wait_for_data_ready(&mut self, sensor_type: SensorType) -> Result<(), Error<E>> {
        const MAX_RETRIES: u8 = 100;
        let mut retries = 0;

        while !self.is_data_ready(sensor_type).await? {
            if retries >= MAX_RETRIES {
                return Err(Error::Timeout);
            }
            self.delay.delay_ms(1).await;
            retries += 1;
        }

        Ok(())
    }

    async fn is_data_ready(&mut self, sensor_type: SensorType) -> Result<bool, Error<E>> {
        let status = self.read_register(Register::STATUS).await?;
        match sensor_type {
            SensorType::Accelerometer => Ok((status & 0b1000_0000) != 0), // Check bit 7 (drdy_acc)
            SensorType::Gyroscope => Ok((status & 0b0100_0000) != 0),     // Check bit 6 (drdy_gyr)
        }
    }
}
