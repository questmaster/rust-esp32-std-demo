use core::{borrow::Borrow, marker::PhantomData};

use esp_idf_sys::*;

use crate::gpio::InputPin;


/// Pins used by the I2S interface
pub struct Pins<
    VSYNC: InputPin,
    HSYNC: InputPin,
    HREF: InputPin,
    PCLK: InputPin,
    SD0: InputPin,
    SD1: InputPin,
    SD2: InputPin,
    SD3: InputPin,
    SD4: InputPin,
    SD5: InputPin,
    SD6: InputPin,
    SD7: InputPin,
> {
    pub vsync: VSYNC,
    pub hsync: HSYNC,
    pub href: HREF,
    pub pclk: PCLK,
    pub sd0: SD0,
    pub sd1: SD1,
    pub sd2: SD2,
    pub sd3: SD3,
    pub sd4: SD4,
    pub sd5: SD5,
    pub sd6: SD6,
    pub sd7: SD7,
}


/// I2S configuration
/*pub mod config {
    use crate::units::*;

    /// I2S configuration
    #[derive(Copy, Clone)]
    pub struct Config {
        pub baudrate: Hertz,
        pub data_mode: embedded_hal::spi::Mode,
    }

    impl Config {
        pub fn new() -> Self {
            Default::default()
        }

        #[must_use]
        pub fn baudrate(mut self, baudrate: Hertz) -> Self {
            self.baudrate = baudrate;
            self
        }

        #[must_use]
        pub fn data_mode(mut self, data_mode: embedded_hal::spi::Mode) -> Self {
            self.data_mode = data_mode;
            self
        }
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                baudrate: Hertz(1_000_000),
                data_mode: embedded_hal::spi::MODE_0,
            }
        }
    }
}*/

struct Lock(i2s_port_t);

impl Lock {
    fn new(port: i2s_port_t) -> Result<Self, EspError> {
        //esp!(unsafe { spi_device_acquire_bus(port, portMAX_DELAY) })?; // TODO fix i2s

        Ok(Self(port))
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        unsafe {
            //spi_device_release_bus(self.0); // TODO fix i2s
        }
    }
}


/// Master SPI abstraction
pub struct CameraSlave<
    I2S: I2s,
    VSYNC: InputPin,
    HSYNC: InputPin,
    HREF: InputPin,
    PCLK: InputPin,
    SD0: InputPin,
    SD1: InputPin,
    SD2: InputPin,
    SD3: InputPin,
    SD4: InputPin,
    SD5: InputPin,
    SD6: InputPin,
    SD7: InputPin,
> {
    i2s: I2S,
    pins: Pins<VSYNC, HSYNC, HREF, PCLK, SD0, SD1, SD2, SD3, SD4, SD5, SD6, SD7>,
    port: i2s_port_t,
}

impl<I2S: I2s, VSYNC: InputPin, HSYNC: InputPin, HREF: InputPin, PCLK: InputPin, 
    SD0: InputPin, SD1: InputPin, SD2: InputPin, SD3: InputPin, SD4: InputPin, SD5: InputPin, SD6: InputPin, SD7: InputPin>
    CameraSlave<I2S, VSYNC, HSYNC, HREF, PCLK, SD0, SD1, SD2, SD3, SD4, SD5, SD6, SD7>
{/*
    /// Internal implementation of new shared by all SPI controllers
    fn new_internal(
        spi: SPI,
        pins: Pins<SCLK, SDO, SDI, CS>,
        config: config::Config,
    ) -> Result<Self, EspError> {
        #[cfg(any(esp_idf_version = "4.4", esp_idf_version_major = "5"))]
        let bus_config = spi_bus_config_t {
            flags: SPICOMMON_BUSFLAG_MASTER,
            sclk_io_num: pins.sclk.pin(),

            data4_io_num: -1,
            data5_io_num: -1,
            data6_io_num: -1,
            data7_io_num: -1,
            __bindgen_anon_1: spi_bus_config_t__bindgen_ty_1 {
                mosi_io_num: pins.sdo.pin(),
                //data0_io_num: -1,
            },
            __bindgen_anon_2: spi_bus_config_t__bindgen_ty_2 {
                miso_io_num: pins.sdi.as_ref().map_or(-1, |p| p.pin()),
                //data1_io_num: -1,
            },
            __bindgen_anon_3: spi_bus_config_t__bindgen_ty_3 {
                quadwp_io_num: -1,
                //data2_io_num: -1,
            },
            __bindgen_anon_4: spi_bus_config_t__bindgen_ty_4 {
                quadhd_io_num: -1,
                //data3_io_num: -1,
            },
            //max_transfer_sz: SPI_MAX_TRANSFER_SIZE,
            ..Default::default()
        };

        #[cfg(not(any(esp_idf_version = "4.4", esp_idf_version_major = "5")))]
        let bus_config = spi_bus_config_t {
            flags: SPICOMMON_BUSFLAG_MASTER,
            sclk_io_num: pins.sclk.pin(),

            mosi_io_num: pins.sdo.pin(),
            miso_io_num: pins.sdi.as_ref().map_or(-1, |p| p.pin()),
            quadwp_io_num: -1,
            quadhd_io_num: -1,

            //max_transfer_sz: SPI_MAX_TRANSFER_SIZE,
            ..Default::default()
        };

        esp!(unsafe {
            spi_bus_initialize(SPI::device(), &bus_config, 0 /*TODO: DMA support*/)
        })?;

        let device_config = spi_device_interface_config_t {
            spics_io_num: pins.cs.as_ref().map_or(-1, |p| p.pin()),
            clock_speed_hz: config.baudrate.0 as i32,
            mode: (if config.data_mode.polarity == embedded_hal::spi::Polarity::IdleHigh {
                2
            } else {
                0
            }) | (if config.data_mode.phase
                == embedded_hal::spi::Phase::CaptureOnSecondTransition
            {
                1
            } else {
                0
            }),
            queue_size: 64,
            ..Default::default()
        };

        let mut device_handle: spi_device_handle_t = ptr::null_mut();

        esp!(unsafe {
            spi_bus_add_device(SPI::device(), &device_config, &mut device_handle as *mut _)
        })?;

        Ok(Self {
            spi,
            pins,
            device: device_handle,
        })
    }

    /// Release and return the raw interface to the underlying SPI peripheral
    #[allow(clippy::type_complexity)]
    pub fn release(self) -> Result<(SPI, Pins<SCLK, SDO, SDI, CS>), EspError> {
        esp!(unsafe { spi_bus_remove_device(self.device) })?;
        esp!(unsafe { spi_bus_free(SPI::device()) })?;

        Ok((self.spi, self.pins))
    }

    fn lock_bus(&mut self) -> Result<Lock, SpiError> {
        Lock::new(self.device).map_err(SpiError::other)
    }

    fn lock_bus_for(&mut self, lock_bus: bool, size: usize) -> Result<Option<Lock>, SpiError> {
        if lock_bus && size > TRANS_LEN {
            Ok(Some(self.lock_bus()?))
        } else {
            Ok(None)
        }
    }

    fn transfer_internal(
        &mut self,
        read: &mut [u8],
        write: &[u8],
        lock_bus: bool,
    ) -> Result<(), SpiError> {
        let _lock = self.lock_bus_for(lock_bus, max(read.len(), write.len()))?;

        let len = max(read.len(), write.len());
        for offset in (0..len).step_by(TRANS_LEN) {
            let read_chunk_end = min(offset + TRANS_LEN, read.len());
            let write_chunk_end = min(offset + TRANS_LEN, write.len());

            if read_chunk_end != write_chunk_end {
                let mut buf = [0_u8; TRANS_LEN];

                let write_ptr = if write_chunk_end < offset + TRANS_LEN {
                    if write_chunk_end > offset {
                        buf[0..write_chunk_end - offset]
                            .copy_from_slice(&write[offset..write_chunk_end]);
                    }

                    buf.as_ptr()
                } else {
                    let chunk = &write[offset..write_chunk_end];

                    chunk.as_ptr()
                };

                let read_ptr = if read_chunk_end < offset + TRANS_LEN {
                    buf.as_mut_ptr()
                } else {
                    let chunk = &mut read[offset..read_chunk_end];

                    chunk.as_mut_ptr()
                };

                let transfer_len = max(read_chunk_end, write_chunk_end) - offset;

                self.transfer_internal_raw(read_ptr, transfer_len, write_ptr, transfer_len)?;

                if read_chunk_end > offset && read_chunk_end < offset + TRANS_LEN {
                    read[offset..read_chunk_end].copy_from_slice(&buf[0..read_chunk_end - offset]);
                }
            } else {
                let read_chunk = &mut read[offset..read_chunk_end];
                let write_chunk = &write[offset..write_chunk_end];

                self.transfer_internal_raw(
                    read_chunk.as_mut_ptr(),
                    read_chunk.len(),
                    write_chunk.as_ptr(),
                    write_chunk.len(),
                )?;
            }
        }

        Ok(())
    }

    fn transfer_inplace_internal(
        &mut self,
        data: &mut [u8],
        lock_bus: bool,
    ) -> Result<(), SpiError> {
        let _lock = self.lock_bus_for(lock_bus, data.len())?;

        let total_len = data.len();
        for offset in (0..data.len()).step_by(TRANS_LEN) {
            let chunk = &mut data[offset..min(offset + TRANS_LEN, total_len)];
            let len = chunk.len();
            let ptr = chunk.as_mut_ptr();

            self.transfer_internal_raw(ptr, len, ptr, len)?;
        }

        Ok(())
    }

    fn write_iter_internal<WI>(&mut self, words: WI) -> Result<(), SpiError>
    where
        WI: IntoIterator<Item = u8>,
    {
        let mut words = words.into_iter();

        let mut buf = [0_u8; TRANS_LEN];

        let mut lock = None;

        loop {
            let mut offset = 0_usize;

            while offset < buf.len() {
                if let Some(word) = words.next() {
                    buf[offset] = word;
                    offset += 1;
                } else {
                    break;
                }
            }

            if offset == 0 {
                break;
            }

            if offset == buf.len() && lock.is_none() {
                lock = Some(self.lock_bus()?);
            }

            let chunk = &mut buf[..offset];
            let ptr = chunk.as_mut_ptr();

            self.transfer_internal_raw(ptr, chunk.len(), ptr, chunk.len())?;
        }

        Ok(())
    }

    fn transfer_internal_raw(
        &mut self,
        read: *mut u8,
        read_len: usize,
        write: *const u8,
        write_len: usize,
    ) -> Result<(), SpiError> {
        let mut transaction = spi_transaction_t {
            flags: 0,
            __bindgen_anon_1: spi_transaction_t__bindgen_ty_1 {
                tx_buffer: write as *const _,
            },
            __bindgen_anon_2: spi_transaction_t__bindgen_ty_2 {
                rx_buffer: read as *mut _,
            },
            length: (write_len * 8) as _,
            rxlength: (read_len * 8) as _,
            ..Default::default()
        };

        esp!(unsafe { spi_device_polling_transmit(self.device, &mut transaction as *mut _) })
            .map_err(SpiError::other)?;

        Ok(())
    }*/
}

pub trait I2s: Send {
    fn port() -> i2s_port_t;
}

macro_rules! impl_i2s {
    ($i2s:ident: $port:expr) => {
        pub struct $i2s(pub ::core::marker::PhantomData<*const ()>);

        impl $i2s {
            /// # Safety
            ///
            /// Care should be taken not to instantiate this I2S instance, if it is already instantiated and used elsewhere
            pub unsafe fn new() -> Self {
                $i2s(::core::marker::PhantomData)
            }
        }

        unsafe impl Send for $i2s {}

        impl I2s for $i2s {
            #[inline(always)]
            fn port() -> i2s_port_t {
                $port
            }
        }
    };
}

impl_i2s!(I2S0: i2s_port_t_I2S_NUM_0);
impl_i2s!(I2S1: i2s_port_t_I2S_NUM_1);

