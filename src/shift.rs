//! Library for interactions with shift registers. This allows for to shift
//! 8-bit values out of or into the Raspberry Pi.

use bindings;

#[derive(Debug, Clone)]
pub enum ShiftOrder {
    LSBFirst = 0,
    MSBFirst = 1,
}

#[derive(Debug)]
pub struct ShiftDevice {
    data_pin: u8,
    clock_pin: u8,
    order: ShiftOrder,
}
impl ShiftDevice {
    pub fn new(data_pin: u8, clock_pin: u8, order: ShiftOrder) -> ShiftDevice {
        ShiftDevice {
            data_pin,
            clock_pin,
            order,
        }
    }
    /// This shifts an 8-bit data value in with the data appearing on the `data_pin`
    /// and the clock being sent out on the `clock_pin`. The data is sampled after
    /// the `clock_pin` goes high. (So `clock_pin` high, sample data, `clock_pin`
    /// low, repeat for 8 bits) The 8-bit value is returned by the function.
    pub fn shift_in(&self) -> u8 {
        unsafe { bindings::shiftIn(self.data_pin, self.clock_pin, self.order.clone() as u8) }
    }

    /// The shifts an 8-bit data `value` out with the data being sent out on
    /// `data_pin` and the clock being sent out on the `clock_pin`. order is as
    /// above. Data is clocked out on the rising or falling edge – ie. `data_pin` is
    /// set, then `clock_pin` is taken high then low – repeated for the 8 bits.
    pub fn shift_out(&self, value: u8) {
        unsafe {
            bindings::shiftOut(self.data_pin,
                               self.clock_pin,
                               self.order.clone() as u8,
                               value);
        }
    }
}
