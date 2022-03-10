
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

pub mod i2s_cam;

pub fn setup (
    sda: gpio::Gpio13<gpio::Unknown>,
    scl: gpio::Gpio12<gpio::Unknown>,
    i2c: i2c::I2C1,
    pwdn: gpio::Gpio26<gpio::Unknown>,
    xclk: gpio::Gpio32<gpio::Unknown>,
    ledc_timer: TIMER0,
    ledc_channel: CHANNEL0,
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
     let i2s = i2s_cam::I2S0;
//     let i2s_pins = i2s_cam::Pins { -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 };
//     let i2s0 = i2s_cam::CameraSlave(i2s, i2s_pins);


     Ok(())
}
