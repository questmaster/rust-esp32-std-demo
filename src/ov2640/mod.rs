use std::io::{Read, Write};
use std::error::Error;

use embedded_hal::prelude::_embedded_hal_blocking_i2c_WriteRead;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::prelude::_embedded_hal_blocking_i2c_Write;
use embedded_hal::digital::v2::OutputPin;

use esp_idf_hal::delay;
use esp_idf_hal::gpio;
use esp_idf_hal::i2c;
use esp_idf_hal::prelude::*;
use esp_idf_hal::ledc;
use esp_idf_hal::ledc::{config::TimerConfig, Channel, Timer, Resolution, TIMER0, CHANNEL0};

use esp_idf_sys::EspError;
use crate::ov2640::i2s_cam::I2sPeripherals;

pub mod i2s_cam;

pub fn setup (
    sda: gpio::Gpio13<gpio::Unknown>,
    scl: gpio::Gpio12<gpio::Unknown>,
    i2c: i2c::I2C1,
    pwdn: gpio::Gpio26<gpio::Unknown>,
    xclk: gpio::Gpio32<gpio::Unknown>,
    ledc_timer: TIMER0,
    ledc_channel: CHANNEL0,
    vsync: gpio::Gpio27<gpio::Unknown>,
    href: gpio::Gpio25<gpio::Unknown>,
    pclk: gpio::Gpio19<gpio::Unknown>,
    sd0: gpio::Gpio5<gpio::Unknown>,
    sd1: gpio::Gpio14<gpio::Unknown>,
    sd2: gpio::Gpio4<gpio::Unknown>,
    sd3: gpio::Gpio15<gpio::Unknown>,
    sd4: gpio::Gpio18<gpio::Unknown>,
    sd5: gpio::Gpio23<gpio::Unknown>,
    sd6: gpio::Gpio36<gpio::Unknown>,
    sd7: gpio::Gpio39<gpio::Unknown>,
) -> Result<(), EspError>
{
    let mut delay = delay::Ets;

     println!("Configuring LEDC output channel");

     let ledc_config = TimerConfig::default().frequency(16.MHz().into())
                                             .resolution(Resolution::Bits1);
     let ledc_timer = Timer::new(ledc_timer, &ledc_config)?;
     let mut ledc_channel = Channel::new(ledc_channel, &ledc_timer, xclk)?;

     let max_duty = ledc_channel.get_max_duty();
     ledc_channel.set_duty(max_duty )?;

     println!("Starting I2C OV2640 test");
 
     let config = <i2c::config::MasterConfig as Default>::default().baudrate(8.kHz().into());
     let mut i2c0 = i2c::Master::<i2c::I2C1, _, _>::new(i2c, i2c::MasterPins { sda, scl }, config)?;
     let mut pwdn = pwdn.into_output()?;
     pwdn.set_low()?; // power-up = low
 
 
     // SCCB_Probe
     /* 
     OV2640_PID = 0x26,
     OV2640_SCCB_ADDR   = 0x30
     */
     let slave_addr = 0x30;
     let ov2640_pid : u8 = 0x26;
     let byte1: &mut [u8; 1] =  &mut [0_u8];
     let byte2: &mut [u8; 1] =  &mut [0_u8];

     i2c0.write(slave_addr, &[0xff, 0x01]);
     let i2c_res1 = i2c0.write(slave_addr, &[0x12, 0x80]);
//        println!("write -> {:#?}", i2c_res1);
     delay.delay_ms(10 as u32);
     i2c0.write_read(slave_addr, &[0x0a], byte1);
     let i2c_res = i2c0.write_read(slave_addr, &[0x0b], byte2);
     println!("return command: PID={} VER={} -> {:#?}", byte1[0], byte2[0], i2c_res);

     if ov2640_pid == byte1[0] {
         println!("OV2640 found");
         
     } else {
         println!("OV2640 mismatch");
         
     }

     // I2S init
     let mut vsync = vsync.into_input()?;
     let mut pclk = pclk.into_input()?;
     let mut sd0 = sd0.into_input()?;
     let mut sd1 = sd1.into_input()?;
     let mut sd2 = sd2.into_input()?;
     let mut sd3 = sd3.into_input()?;
     let mut sd4 = sd4.into_input()?;
     let mut sd5 = sd5.into_input()?;
     let mut sd6 = sd6.into_input()?;
     let mut sd7 = sd7.into_input()?;

     let i2speriph = I2sPeripherals::take().unwrap();
     let i2s = i2speriph.i2s0;
     let i2s_config = <i2s_cam::config::Config as Default>::default();
     let i2s_pins = i2s_cam::Pins { vsync, href, pclk, sd0, sd1, sd2, sd3, sd4, sd5, sd6, sd7 };
     let i2s0 = i2s_cam::CameraSlave::<i2s_cam::I2S0, _, _, _, _, _, _, _, _, _, _, _>::new(i2s, i2s_pins, i2s_config);


     Ok(())
}
