#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use stm32h5::stm32h563;

use bmi323::{
    Bmi323, AccelConfig, GyroConfig, OutputDataRate, AccelerometerRange, GyroscopeRange,
    Bandwidth, AverageNum, AccelerometerPowerMode, GyroscopePowerMode,
};

use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::{ErrorKind, ErrorType, Error as OtherError, SevenBitAddress};
use embedded_hal::i2c::Operation;

#[derive(Debug)]
pub enum I2cError {
    NullPtr,
    NackReceived,
    BusError,
    ArbitrationLoss,
    OverrunError,
    DmaError,
    Timeout,
}

impl OtherError for I2cError {
    fn kind(&self) -> ErrorKind {
        match *self {
            I2cError::NullPtr => ErrorKind::Other,
            I2cError::NackReceived => ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Unknown),
            I2cError::BusError => ErrorKind::Bus,
            I2cError::ArbitrationLoss => ErrorKind::ArbitrationLoss,
            I2cError::OverrunError => ErrorKind::Overrun,
            I2cError::DmaError => ErrorKind::Other,
            I2cError::Timeout => ErrorKind::Other,
        }
    }
}

struct I2C1 {
    i2c: stm32h563::I2C1,
}

impl ErrorType for I2C1 {
    type Error = I2cError;
}

impl embedded_hal::i2c::I2c<SevenBitAddress> for I2C1 {
    fn transaction(
        &mut self,
        address: SevenBitAddress,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        for op in operations {
            match op {
                Operation::Read(buffer) => {
                    self.read(address, buffer)?;
                }
                Operation::Write(buffer) => {
                    self.write(address, buffer)?;
                }
            }
        }
        Ok(())
    }
}

struct Delay;

impl DelayNs for Delay {
    fn delay_us(&mut self, us: u32) {
        bmi323_delay(us);
    }

    fn delay_ns(&mut self, ns: u32) {
        bmi323_delay(ns);
    }

    fn delay_ms(&mut self, ms: u32) {
        bmi323_delay(ms);
    }

}

#[entry]
fn main() -> ! {
    let peripherals = unsafe { stm32h563::Peripherals::steal() };
    init_device(&peripherals);

    if i2c1_init(&peripherals).is_err() {
        loop {}
    }

    let i2c = I2C1 {
        i2c: peripherals.I2C1,
    };
    let delay = Delay;
    let mut bmi323 = Bmi323::new_with_i2c(i2c, 0x68, delay);

    bmi323.init().unwrap();

    let accel_config = AccelConfig::builder()
        .odr(OutputDataRate::Odr100hz)
        .range(AccelerometerRange::G16)
        .bw(Bandwidth::OdrQuarter) // ODR/4
        .avg_num(AverageNum::Avg1)
        .mode(AccelerometerPowerMode::Normal)
        .build();

    let gyro_config = GyroConfig::builder()
        .odr(OutputDataRate::Odr100hz)
        .range(GyroscopeRange::DPS2000)
        .bw(Bandwidth::OdrHalf) // ODR/2
        .avg_num(AverageNum::Avg1)
        .mode(GyroscopePowerMode::Normal)
        .build();

    bmi323.set_accel_config(accel_config).unwrap();
    
    bmi323.set_gyro_config(gyro_config).unwrap();
    // if set_config(&mut bmi323).is_err() {
    //     loop {}
    // }

    let mut data_gyr_arr = [bmi323::Sensor3DDataScaled { x: 0.0, y: 0.0, z: 0.0 }; 100];
    let mut data_acc_arr = [bmi323::Sensor3DDataScaled { x: 0.0, y: 0.0, z: 0.0 }; 100];
    let mut count = 0;

    loop {
        while count < 100 {
            if let Ok(accel_data) = bmi323.read_accel_data_scaled() {
                data_acc_arr[count] = accel_data;
                
            }

            if let Ok(gyro_data) = bmi323.read_gyro_data_scaled() {
                data_gyr_arr[count] = gyro_data;
                
            }
            count += 1;
        }
        
        // Process or transmit the collected data here
        
    }
}

fn bmi323_delay(period: u32) {
    for _ in 0..period {
        cortex_m::asm::nop();
    }
}
  
impl I2C1 {
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), I2cError> {
        let i2c = &self.i2c;

        // Configure I2C for reading data
        i2c.cr2().write(|w| unsafe {
            w.sadd().bits((address << 1) as u16)
             .rd_wrn().set_bit() // Set to read mode
             .nbytes().bits(buffer.len() as u8)
             .start().set_bit()
             .autoend().set_bit()
        });

        // Read bytes and store them in the buffer
        for i in 0..buffer.len() {
            // Wait until RXNE flag is set
            while i2c.isr().read().rxne().bit_is_clear() {}
            // Read data from the receive data register
            buffer[i] = i2c.rxdr().read().rxdata().bits();
        }

        // Wait until STOP flag is set
        while i2c.isr().read().stopf().bit_is_clear() {}
        // Clear the STOP flag
        i2c.icr().write(|w| w.stopcf().set_bit());

        Ok(())
    }


    fn write(&mut self, address: u8, buffer: &[u8]) -> Result<(), I2cError> {
        let i2c = &self.i2c;

        // Configuring the I2C peripheral: START condition, device address, length, and AUTOEND
        i2c.cr2().write(|w| unsafe {
            w.sadd().bits((address << 1) as u16)
             .nbytes().bits(buffer.len() as u8)
             .start().set_bit()
             .autoend().set_bit()
        });

        // Write bytes from the buffer to the transmit data register
        for &byte in buffer.iter() {
            // Wait until TXIS flag is set
            while i2c.isr().read().txis().bit_is_clear() {}
            // Write data to the transmit data register
            i2c.txdr().write(|w| unsafe { w.txdata().bits(byte) });
        }

        // Wait until STOP flag is set
        while i2c.isr().read().stopf().bit_is_clear() {}
        // Clear the STOP flag
        i2c.icr().write(|w| w.stopcf().set_bit());

        // Check for NACK
        if i2c.isr().read().nackf().bit_is_set() {
            return Err(I2cError::NackReceived);
        }

        Ok(())
    }
}

fn i2c1_init(peripherals: &stm32h563::Peripherals) -> Result<(), I2cError> {
    // i2c1 addressing mode is 7-bit
    let i2c1 = &peripherals.I2C1;
    let rcc = &peripherals.RCC;
    let gpiob = &peripherals.GPIOB;
    // let mut rslt: i8 = 0;

    // rcc.cr().read()
    // rcc.pll1cfgr().modify(|_, w| w.divm1().bits(1));

    rcc.ahb2enr().modify(|_, w| w.gpioben().set_bit());
    rcc.apb1lenr().modify(|_, w| w.i2c1en().set_bit());
    // Configure GPIOB 6 (SCL) and GPIOB 7 (SDA) for I2C1
    // Set mode to alternate function (10)
    gpiob
        .moder()
        .modify(|_, w| w.mode6().alternate().mode7().alternate());
    // Set output type to open-drain (1)
    gpiob
        .otyper()
        .modify(|_, w| w.ot6().set_bit().ot7().set_bit());
    // Set pull-up (01)
    gpiob
        .pupdr()
        .modify(|_, w| w.pupd6().pull_up().pupd7().pull_up());
    // Set speed to high (10 or 11)
    gpiob
        .ospeedr()
        .modify(|_, w| w.ospeed6().high_speed().ospeed7().high_speed());
    // Set alternate function to I2C1 (AF4 for STM32H5 series)
    gpiob
        .afrl()
        .modify(|_, w| unsafe { w.afsel6().bits(0x4).afsel7().bits(0x4) });

    //read the register to ensure the write has taken effect
    // let test = gpiob.afrl().read().afsel6().bits() as i8;

    i2c1.cr2().modify(|_, w| w.add10().clear_bit());

    i2c1.timingr().modify(|_, w| unsafe { w.bits(0x00707CBB) });

    // read the register to ensure the write has taken effect
    let timr_test = i2c1.timingr().read().bits();

    i2c1.cr1().modify(|_, w| w.pe().set_bit());

    if i2c1.cr1().read().pe().bit_is_set() && timr_test == 0x00707CBB {
        Ok(()) // Success
    } else {
        Err(I2cError::BusError) // Failure
    }
}

fn init_device(peripherals: &stm32h563::Peripherals) {
    // Get access to the device specific peripherals
    // let dp = peripherals;

    // RCC: Reset and Clock Control
    let rcc = &peripherals.RCC;
    // Enable HSI oscillator and select it as SYSCLK source
    rcc.cr().modify(|_, w| w.hsion().set_bit());
    while rcc.cr().read().hsirdy().bit_is_clear() {}
    rcc.cfgr1().modify(|_, w| unsafe { w.sw().bits(0) });

    // Wait for HSI to be used as the system clock
    while rcc.cfgr1().read().sw().bits() != 0 {}

    rcc.cfgr1().modify(|_, w| w.timpre().set_bit()); // AHB prescaler of 2 (64 MHz / 2 = 32 MHz

    // Configure the HSI as SYSCLK source and configure the clocks
    // HSI is 64 MHz, we divide it by 2 to get 32 MHz SYSCLK
    rcc.cr()
        .modify(|_, w| unsafe { w.hsikeron().set_bit().hsidiv().bits(1) });
    // rcc.cfgr().modify(|_, w| unsafe { w. });

    // Configure flash wait states for 32 MHz SYSCLK
    // flash.acr().modify(|_, w| w.latency().ws2());
    // I2C1SEL[1:0]: I2C1 kernel clock source selection
    // 00: rcc_pclk1 selected as kernel clock (default after reset)
    // 01: pll3_r_ck selected as kernel clock
    // 10: hsi_ker_ck selected as kernel clock
    // 11: csi_ker_ck selected as kernel clock
    // Optional: Configure I2C1 clock source to HSI (which is now 32 MHz)
    rcc.ccipr4().modify(|_, w| unsafe { w.i2c1sel().bits(10) });
    // rcc.ccipr4().modify(|_, w| unsafe { w.i3c1sel().bits(10) });
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}