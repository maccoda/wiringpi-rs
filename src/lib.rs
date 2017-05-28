extern crate wiringPi_bindings as bindings;
#[macro_use]
extern crate error_chain;

use std::sync::{Once, ONCE_INIT};

pub mod constants;

use constants::*;

error_chain!{
    errors {
        IncorrectConfiguration(t: String)
        IllegalPinMode(t: String)
    }
}

#[derive(Debug, Clone)]
pub enum WiringPiConfiguration {
    /// Sets up **wiringPi** and uses the wiringPi numbering scheme for pins.
    Def,
    /// Sets up **wiringPi** using the Broadcom GPIO pin numbers directly.
    Gpio,
    /// Sets up **wiringPi** using the Broadcom GPIO pin number but restricts
    /// the calling program to using pins on the *P1 connector only*.
    Phys,
    /// Initializes **wiringPi** but uses the */sys/class/gpio* interface rather
    /// than accessing the hardware directly. This is the only configuration
    /// that will allow a user to not have root privelages, provided the GPIO
    /// pins have been exported before-hand using the `gpio` program.
    Sys,
}

/// Access structure into the wiringPi library.
#[derive(Debug)]
pub struct WiringPi {
    config: WiringPiConfiguration,
}

// One time initialization primitive to allow for only one time set up of the
// wiringPi library
static LIB_INIT: Once = ONCE_INIT;

impl WiringPi {
    /// Setup the wiringPi library according to the configuration provided. The
    /// library can only be setup once in the program.
    pub fn new(config: WiringPiConfiguration) -> WiringPi {
        unsafe {
            LIB_INIT.call_once(|| {
                                   let _ = match config {
                                       WiringPiConfiguration::Def => bindings::wiringPiSetup(),
                                       WiringPiConfiguration::Gpio => bindings::wiringPiSetupGpio(),
                                       WiringPiConfiguration::Phys => bindings::wiringPiSetupPhys(),
                                       WiringPiConfiguration::Sys => {
                                           println!("NOTE: Cannot change the mode if the pin. To do so need to change via gpio program.");
                                           bindings::wiringPiSetupSys()},
                                   };
                               })
        }
        WiringPi { config }
    }

    /// Obtain access to the pin at the specified number.
    pub fn pin(&self, pin_number: u32) -> Pin {
        Pin {
            number: pin_number,
            mode: PinModes::Output,
            config: self.config.clone(),
        }
    }

    /// Writes the byte provided to the first 8 GPIO pins. This is a more
    /// efficient way of writing to all.
    pub fn digital_write_byte(&self, value: i32) {
        // TODO cannot be done in sys mode
        unsafe {
            bindings::digitalWriteByte(value);
        }
    }

    /// Set the mode of the PWM generator to that supplied.
    pub fn pwm_set_mode(&self, mode: PwmMode) {
        // TODO cannot be done in sys mode
        unsafe {
            bindings::pwmSetMode(mode.ordinal());
        }
    }

    /// Set the range register of the PWM generator.
    ///
    /// The default value is **1024**.
    pub fn pwm_set_range(&self, range: u32) {
        // TODO cannot be done in sys mode
        unsafe {
            bindings::pwmSetRange(range);
        }
    }

    /// Sets the clock divisor to that provided.
    pub fn pwm_set_clock(&self, divisor: i32) {
        // TODO cannot be done in sys mode
        unsafe {
            bindings::pwmSetClock(divisor);
        }
    }

    /// Returns the board revision of the Raspberry Pi. This is most applicable
    /// when configured to be using Broadcom GPIO numbering as they have changed
    /// from revisions.
    pub fn pi_board_rev() -> i32 {
        // TODO look into how this can be made into an enum
        unsafe { bindings::piBoardRev() }
    }

    /// Maps the provided wiringPi pin number to the Broadcom GPIO pin number.
    pub fn wpi_pin_to_gpio(pin_number: i32) -> i32 {
        unsafe { bindings::wpiPinToGpio(pin_number) }
    }

    /// Maps the provided physical pin on the **P1** connector to the Broadcom
    /// GPIO pin number.
    pub fn phys_pin_to_gpio(pin_numer: i32) -> i32 {
        unsafe { bindings::physPinToGpio(pin_numer) }
    }

    /// Sets the "strength" of the pad drivers for the provided group of pins.
    ///
    /// There are 3 groups of pins and the drive strength is from 0 to 7.
    pub fn set_pad_drive(group: i32, value: i32) {
        unsafe { bindings::setPadDrive(group, value) }
    }
}



/// Structure representing a physical GPIO pin on the Raspberyy Pi.
#[derive(Debug)]
pub struct Pin {
    // TODO Consider using more typing rather than internal state to limit the
    // number of functions. This will make for simpler code and more cohert
    // usage of the library.
    //
    // TODO Create a pin number type that is chosen based
    // on the configuration and ensures that the number is within range.
    number: u32,
    mode: PinModes,
    config: WiringPiConfiguration,
}

impl Pin {
    /// Set the mode of the `Pin` to that specified.
    ///
    /// # Errors
    ///
    /// * If configuration is [`WiringPiConfiguration::Sys`] this
    /// function will have no effect
    ///
    /// [`WiringPiConfiguration::Sys`]: enum.WiringPiConfiguration.html
    pub fn set_pin_mode(&mut self, mode: PinModes) -> Result<()> {
        // TODO Add logic to ensure that the number of the pin matches the mode
        // attempting to set it to
        self.check_config()
            .and_then(|_| {
                          unsafe {
                              bindings::pinMode(self.number as i32, mode.ordinal());
                          }
                          self.mode = mode;
                          Ok(())
                      })
    }

    /// Set the resistor mode of the `Pin` to that specified.
    ///
    /// # Errors
    ///
    /// * If configuration is [`WiringPiConfiguration::Sys`] this
    /// function will have no effect
    /// * If current mode of the pin is not [`Input`]
    ///
    ///
    /// [`WiringPiConfiguration::Sys`]: enum.WiringPiConfiguration.html
    /// [`Input`]: constants/enum.PinModes.html#variants
    pub fn set_resistor_mode(&mut self, mode: ResistorMode) -> Result<()> {
        self.check_config()
            .and_then(|_| match self.mode {
                          PinModes::Input(_) => {
                              unsafe {
                                  bindings::pullUpDnControl(self.number as i32, mode.ordinal());
                              }
                              self.mode = PinModes::Input(mode);
                              Ok(())
                          }
                          _ => {
                Err(ErrorKind::IllegalPinMode("Cannot set resistor mode as not input mode".into())
                        .into())
            }
                      })
    }

    /// Checks if Sys configuration as cannot change the mode of pins in this
    /// configuration
    fn check_config(&self) -> Result<()> {
        if let WiringPiConfiguration::Sys = self.config {
            return Err(ErrorKind::IncorrectConfiguration(
                "Unable to change pin mode in sys configuration. This needs to be done via the gpio
                 program before the software starts".into()).into());
        }
        Ok(())
    }

    /// Writes the given value to the pin.
    ///
    /// # Errors
    ///
    /// * If current pin mode is not [`Output`]
    ///
    /// [`Output`]: constants/enum.PinModes.html#variants
    pub fn digital_write(&self, value: DigitalOut) -> Result<()> {
        match self.mode {
            PinModes::Output => {
                unsafe {
                    bindings::digitalWrite(self.number as i32, value.ordinal());
                }
                Ok(())
            }
            _ => {
                Err(ErrorKind::IllegalPinMode("Cannot write to pin not in output mode".into())
                        .into())
            }
        }
    }

    /// Reads the logic level at the pin.
    pub fn digital_read(&self) -> DigitalOut {
        // TODO Should ensure in the correct mode
        unsafe {
            let result = bindings::digitalRead(self.number as i32);
            result.into()
        }
    }

    /// Writes the given value to the PWM register for the pin.
    ///
    /// Note that Raspberry Pi has one on-board PWM pin.
    ///
    /// # Errors
    ///
    /// * If configuration is [`WiringPiConfiguration::Sys`] this
    /// function will have no effect
    ///
    /// [`WiringPiConfiguration::Sys`]: enum.WiringPiConfiguration.html
    pub fn pwm_write(&self, value: u32) -> Result<()> {
        // TODO should throw error when not the correct pin
        self.check_config()
            .and_then(|_| {
                          unsafe {
                              bindings::pwmWrite(self.number as i32, value as i32);
                          }
                          Ok(())
                      })
    }

    /// Returns the value read on the current analog input pin.
    pub fn analog_read(&self) -> i32 {
        // TODO Should ensure in the correct mode
        unsafe { bindings::analogRead(self.number as i32) }
    }

    pub fn analog_write(&self, value: i32) {
        // TODO Should ensure in the correct mode
        unsafe {
            bindings::analogWrite(self.number as i32, value);
        }
    }
}
