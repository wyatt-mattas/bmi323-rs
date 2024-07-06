#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::i2c::{Config, I2c};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use {defmt_rtt as _, panic_probe as _};

mod hal_compat;
use hal_compat::{EmbassyI2cWrapper, EmbassyDelayWrapper};

use bmi323::{AccelConfig, Bmi323, GyroConfig, OutputDataRate, AccelerometerRange, GyroscopeRange, AverageNum, Bandwidth, AccelerometerPowerMode, GyroscopePowerMode, Sensor3DDataScaled};

const BMI323_ADDRESS: u8 = 0x68;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // info!("Hello world!");
    let p = embassy_stm32::init(Default::default());
    let mut i2c_config = Config::default();
    i2c_config.scl_pullup = true;
    i2c_config.sda_pullup = true;
    let i2c = I2c::new(
        p.I2C1,
        p.PB6,
        p.PB7,
        Irqs,
        p.GPDMA1_CH4,
        p.GPDMA1_CH5,
        Hertz(100_000),
        i2c_config,
    );

    let i2c_wrapper = EmbassyI2cWrapper(i2c);
    let delay_wrapper = EmbassyDelayWrapper;

    // Create BMI323 instance
    let mut bmi323 = Bmi323::new_with_i2c(i2c_wrapper, BMI323_ADDRESS, delay_wrapper);

    // Initialize BMI323
    match bmi323.init() {
        Ok(_) => info!("BMI323 initialized successfully"),
        Err(_) => info!("BMI323 initialized error"),
    }

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

    if let Err(e) = bmi323.set_accel_config(accel_config) {
        info!("Failed to configure accelerometer: {:?}", e);
    }
    
    if let Err(e) = bmi323.set_gyro_config(gyro_config) {
        info!("Failed to configue gyroscope: {:?}", e);
    }

    let mut data_gyr_arr: [Sensor3DDataScaled; 100] = [Sensor3DDataScaled { x: 0.0, y: 0.0, z: 0.0 }; 100];
    let mut data_acc_arr: [Sensor3DDataScaled; 100] = [Sensor3DDataScaled { x: 0.0, y: 0.0, z: 0.0 }; 100];
    let mut count = 0;
    loop {
        while count < 100 {
    
            match bmi323.read_accel_data_scaled() {
                Ok(data) => {
                    info!("Accel data: x={}, y={}, z={}", data.x, data.y, data.z);
                    data_acc_arr[count as usize] = data;
                },
                Err(e) => info!("Failed to read accelerometer data: {:?}", e),
            }
    
            match bmi323.read_gyro_data_scaled() {
                Ok(data) => {
                    info!("Gyro data: x={}, y={}, z={}", data.x, data.y, data.z);
                    data_gyr_arr[count as usize] = data;
                },
                Err(e) => info!("Failed to read gyroscope data: {:?}", e),
            }
    
            count += 1;
        }
    }
}

// fn bmi323_delay(period: u32) {
//     for _ in 0..period {
//         cortex_m::asm::nop();
//     }
// }
