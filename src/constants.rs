//! Module exporting all constants used in wiringPi

/// Mode of GPIO pin.
#[derive(Debug)]
pub enum PinModes {
    Input(ResistorMode),
    Output,
    PwmOutput,
    GpioClock,
    SoftPwmOutput,
    SoftToneOutput,
    PwmToneOutput,
}
impl Ordinal for PinModes {
    fn ordinal(&self) -> i32 {
        match self {
            &PinModes::Input(_) => 0,
            &PinModes::Output => 1,
            &PinModes::PwmOutput => 2,
            &PinModes::GpioClock => 3,
            &PinModes::SoftPwmOutput => 4,
            &PinModes::SoftToneOutput => 5,
            &PinModes::PwmToneOutput => 6,
        }
    }
}

/// Possible values to write to a digital output pin.
#[derive(Debug)]
pub enum DigitalOut {
    Low,
    High,
}

impl Ordinal for DigitalOut {
    fn ordinal(&self) -> i32 {
        match self {
            &DigitalOut::Low => 0,
            &DigitalOut::High => 1,
        }
    }
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
    None,
    /// Pull to ground
    PullDown,
    /// Pull to *VDD* (3.3V)
    PullUp,
}

impl Ordinal for ResistorMode {
    fn ordinal(&self) -> i32 {
        match self {
            &ResistorMode::None => 0,
            &ResistorMode::PullDown => 1,
            &ResistorMode::PullUp => 2,
        }
    }
}

/// Possible modes that the PWM generator can be run in.
#[derive(Debug)]
pub enum PwmMode {
    /// Default mode of the PWM on the Raspberry Pi
    Balanced,
    MarkSpace,
}

impl Ordinal for PwmMode {
    fn ordinal(&self) -> i32 {
        match self {
            &PwmMode::MarkSpace => 0,
            &PwmMode::Balanced => 1,
        }
    }
}

#[derive(Debug)]
pub enum InterruptEdgeType {
    EdgeSetup,
    FallingEdge,
    RisingEdge,
    BothEdges,
}

impl Ordinal for InterruptEdgeType {
    fn ordinal(&self) -> i32 {
        match self {
            &InterruptEdgeType::EdgeSetup => 0,
            &InterruptEdgeType::FallingEdge => 1,
            &InterruptEdgeType::RisingEdge => 2,
            &InterruptEdgeType::BothEdges => 3,
        }
    }
}

/// Trait to allow for enumerations to be converted to their ordinal
/// representation within the enumeration.
///
/// This has the main purpose of being able to interact with the underlying C
/// constants in a more strongly typed manner.
pub trait Ordinal {
    /// Returns the ordinal of the enumeration
    fn ordinal(&self) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;
    // Keeping this here in case decide to write a macro for the ordinal trait
    #[test]
    fn test_ordinal_pin_mode() {
        assert_eq!(0, PinModes::Input(ResistorMode::None).ordinal());
        assert_eq!(1, PinModes::Output.ordinal());
        assert_eq!(2, PinModes::PwmOutput.ordinal());
        assert_eq!(3, PinModes::GpioClock.ordinal());
        assert_eq!(4, PinModes::SoftPwmOutput.ordinal());
        assert_eq!(5, PinModes::SoftToneOutput.ordinal());
        assert_eq!(6, PinModes::PwmToneOutput.ordinal());
    }
}
