//! Library to simplify interactions with the on-board I2C interface of the
//! Raspberry Pi.
//!
//! Before this can be used the **gpio** facility needs to be set up by
//! performing the following:
//!
//! ```
//! gpio load i2c
//! ```
use super::FileDescriptor;
use super::{Result, ErrorKind};
use bindings;

#[derive(Debug)]
pub struct I2CDevice {
    descriptor: FileDescriptor,
}

impl I2CDevice {
    /// Initializes the system with the provided I2C identifier. The ID is that
    /// of the device to interact with and can be detected using **i2cdetect**.
    pub fn setup(dev_id: i32) -> Result<I2CDevice> {
        unsafe {
            let result = bindings::wiringPiI2CSetup(dev_id);
            if result == -1 {
                Err(ErrorKind::WiringPiFail.into())
            } else {
                Ok(I2CDevice { descriptor: FileDescriptor(result) })
            }
        }
    }

    /// Simple device read
    pub fn read(&self) -> i32 {
        unsafe { bindings::wiringPiI2CRead(self.descriptor.0) }
    }

    /// Simple device write
    pub fn write(&self, data: i32) -> Result<()> {
        unsafe {
            let result = bindings::wiringPiI2CWrite(self.descriptor.0, data);
            if result < 0 {
                Err(ErrorKind::WiringPiFail.into())
            } else {
                Ok(())
            }
        }
    }
}
