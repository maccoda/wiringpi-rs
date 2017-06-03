//! Binding to the serial library provided by wiringPi. This provides a
//! simplified serial port handling library.
//!
//! This maps to the functionality provided by `wiringSerial.h` and is enabled
//! through the *serial* feature.
use std::ffi::CString;
use std::os::raw::{c_uchar, c_int};

use super::{ErrorKind, Result};
use bindings;
use super::FileDescriptor;

#[derive(Debug)]
pub struct SerialDevice {
    descriptor: FileDescriptor,
}

impl SerialDevice {
    /// Opens and initializes the serial device and sets the Baud rate to that
    /// specified. Sets the read timeout to **10 seconds**.
    pub fn serial_open(device_name: String, baud_rate: i32) -> Result<SerialDevice> {
        unsafe {
            let c_string = CString::new(device_name).unwrap();
            let result = bindings::serialOpen(c_string.into_raw(), baud_rate);
            if result == -1 {
                Err(ErrorKind::WiringPiFail.into())
            } else {
                Ok(SerialDevice { descriptor: FileDescriptor(result) })
            }
        }
    }

    /// Closes the serial device.
    pub fn serial_close(&self) {
        unsafe {
            bindings::serialClose(self.descriptor.0);
        }
    }

    /// Sends a single byte to the serial device.
    pub fn put_char(&self, character: c_uchar) {
        unsafe {
            bindings::serialPutchar(self.descriptor.0, character);
        }
    }

    /// Sends the null terminated string to the serial device.
    pub fn put_string(&self, string: String) {
        unsafe {
            bindings::serialPuts(self.descriptor.0, CString::new(string).unwrap().into_raw());
        }
    }

    // TODO serialPrintf

    /// Returns the number of characters available.
    pub fn data_available(&self) -> Result<u32> {
        unsafe {
            let result = bindings::serialDataAvail(self.descriptor.0);
            if result == -1 {
                Err(ErrorKind::WiringPiFail.into())
            } else {
                Ok(result as u32)
            }
        }
    }

    /// Returns the next character available on the serial device.
    ///
    /// This will block for up to **10 seconds** if no data is available.
    pub fn get_char(&self) -> Result<c_int> {
        unsafe {
            let result = bindings::serialGetchar(self.descriptor.0);
            if result == -1 {
                Err(ErrorKind::WiringPiFail.into())
            } else {
                Ok(result)
            }
        }
    }

    /// Discards all data received or waiting to be sent to the serial device.
    pub fn flush(&self) {
        unsafe {
            bindings::serialFlush(self.descriptor.0);
        }
    }
}
