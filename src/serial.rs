//! Binding to the serial library provided by wiringPi. This provides a
//! simplified serial port handling library.
//!
//! This maps to the functionality provided by `wiringSerial.h` and is enabled
//! through the *serial* feature.
use std::ffi::CString;

use super::{ErrorKind, Result};
use bindings;

pub struct FileDescriptor(i32);
/// Opens and initializes the serial device and sets the Baud rate to that
/// specified. Sets the read timeout to **10 seconds**.
pub fn serial_open(device_name: String, baud_rate: i32) -> Result<FileDescriptor> {
    unsafe {
        let c_string = CString::new(device_name).unwrap();
        let result = bindings::serialOpen(c_string.into_raw(), baud_rate);
        if result == -1 {
            Err(ErrorKind::WiringPiFail.into())
        } else {
            Ok(FileDescriptor(result))
        }
    }
}
