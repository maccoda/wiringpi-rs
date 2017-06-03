//! Library to simplify interactions with the on-board SPI interface of the
//! Raspberry Pi.
//!
//! Before this can be used the **gpio** facility needs to be set up by
//! performing the following:
//!
//! ```
//! gpio load spi
//! ```
use super::{Result, ErrorKind};
use bindings;

#[derive(Debug, Clone)]
pub enum SpiChannel {
    Channel1 = 1,
    Channel2 = 2,
}

#[derive(Debug)]
pub struct SpiDevice {
    channel: u8,
}

impl SpiDevice {
    /// This is the way to initialise a channel (The Pi has 2 channels; 0 and
    /// 1). The speed parameter is an integer in the range 500,000 through
    /// 32,000,000 and represents the SPI clock speed in Hz.
    pub fn setup(channel: SpiChannel, speed: i32) -> Result<SpiDevice> {
        unsafe {
            let result = bindings::wiringPiSPISetup(channel.clone() as i32, speed);
            if result < 0 {
                Err(ErrorKind::WiringPiFail.into())
            } else {
                Ok(SpiDevice { channel: channel as u8 })
            }
        }
    }

    /// This performs a simultaneous write/read transaction over the selected
    /// SPI bus. The returned value buffer will match the size of the provided
    /// buffer.
    pub fn read_write(&self, data: &Vec<u8>) -> Vec<u8> {
        let mut clone_data = data.clone();
        unsafe {
            bindings::wiringPiSPIDataRW(self.channel as i32,
                                        clone_data.as_mut_ptr(),
                                        data.len() as i32);
        }
        clone_data
    }
}
