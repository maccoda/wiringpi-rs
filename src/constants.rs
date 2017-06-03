//! Module exporting all constants used in wiringPi

/// Mode of GPIO pin.
#[derive(Debug, Clone)]
pub enum PinModes {
    Input = 0,
    Output = 1,
    PwmOutput = 2,
    GpioClock = 3,
    // SoftPwmOutput = 4,
    // SoftToneOutput = 5,
    // PwmToneOutput = 6,
}

/// Possible values to write to a digital output pin.
#[derive(Debug)]
pub enum DigitalOut {
    Low = 0,
    High = 1,
}

impl From<i32> for DigitalOut {
    fn from(val: i32) -> DigitalOut {
        if val == 0 {
            DigitalOut::Low
        } else {
            DigitalOut::High
        }
    }
}

/// Resistor mode to specify for a pin.
#[derive(Debug)]
pub enum ResistorMode {
    /// No pull up or pull down
    None = 0,
    /// Pull to ground
    PullDown = 1,
    /// Pull to *VDD* (3.3V)
    PullUp = 2,
}


/// Possible modes that the PWM generator can be run in.
#[derive(Debug)]
pub enum PwmMode {
    /// Default mode of the PWM on the Raspberry Pi
    MarkSpace = 0,
    Balanced = 1,
}

#[derive(Debug)]
pub enum InterruptEdgeType {
    EdgeSetup = 0,
    FallingEdge = 1,
    RisingEdge = 2,
    BothEdges = 3,
}
