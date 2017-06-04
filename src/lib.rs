extern crate wiringPi_bindings as bindings;
#[macro_use]
extern crate error_chain;

use std::sync::{Once, ONCE_INIT};

pub mod constants;

#[derive(Debug)]
struct FileDescriptor(i32);

// #[cfg(feature = "serial")]
pub mod serial;
pub mod i2c;
pub mod spi;
pub mod shift;

use constants::*;

error_chain!{
    errors {
        IncorrectConfiguration(t: String)
        WiringPiFail
    }
}

/// Posible configurations for initializing the wiringPi library.
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
            number: pin_number as u8,
            config: self.config.clone(),
        }
    }

    /// Writes the byte provided to the first 8 GPIO pins. This is a more
    /// efficient way of writing to all.
    ///
    /// # Note
    ///
    /// * If configuration is [`WiringPiConfiguration::Sys`] this
    /// function will have no effect
    ///
    /// [`WiringPiConfiguration::Sys`]: enum.WiringPiConfiguration.html
    pub fn digital_write_byte(&self, value: i32) {
        // TODO cannot be done in sys mode
        unsafe {
            bindings::digitalWriteByte(value);
        }
    }

    /// Set the mode of the PWM generator to that supplied.
    ///
    /// # Note
    ///
    /// * If configuration is [`WiringPiConfiguration::Sys`] this
    /// function will have no effect
    ///
    /// [`WiringPiConfiguration::Sys`]: enum.WiringPiConfiguration.html
    pub fn pwm_set_mode(&self, mode: PwmMode) {
        // TODO cannot be done in sys mode
        unsafe {
            bindings::pwmSetMode(mode as i32);
        }
    }

    /// Set the range register of the PWM generator.
    ///
    /// The default value is **1024**.
    ///
    /// # Note
    ///
    /// * If configuration is [`WiringPiConfiguration::Sys`] this
    /// function will have no effect
    ///
    /// [`WiringPiConfiguration::Sys`]: enum.WiringPiConfiguration.html
    pub fn pwm_set_range(&self, range: u32) {
        // TODO cannot be done in sys mode
        unsafe {
            bindings::pwmSetRange(range);
        }
    }

    /// Sets the clock divisor to that provided.
    ///
    /// # Note
    ///
    /// * If configuration is [`WiringPiConfiguration::Sys`] this
    /// function will have no effect
    ///
    /// [`WiringPiConfiguration::Sys`]: enum.WiringPiConfiguration.html
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

    /// Number of milliseconds (10^-3) since wiringPi library was initialized.
    ///
    /// This number will wrap after **49 days**.
    pub fn millis(&self) -> u32 {
        unsafe { bindings::millis() }
    }

    /// Number of microseconds (10^-6) since wiringPi library was initialized.
    ///
    /// This number will wrap after approximately **71 minutes**.
    pub fn micros(&self) -> u32 {
        unsafe { bindings::micros() }
    }

    /// Pauses program execution for the given number of *milliseconds*.
    ///
    /// Note the maximum amount of delay is approximately 29 days.
    pub fn delay(&self, delay_millis: u32) {
        unsafe { bindings::delay(delay_millis) }
    }

    /// Pauses program execution for the given number of *microseconds*.
    ///
    /// Note the maximum amount of delay is approximately 71 minutes.
    ///
    /// Delays under 100 microseconds are timed using a hard-coded loop
    /// continually polling the system time. Delays greater than 100
    /// microseconds are done using the system `nanosleep` function. It is worth
    /// noting the implications of these on the overall performance.
    pub fn delay_micro(&self, delay_micros: u32) {
        unsafe { bindings::delayMicroseconds(delay_micros) }
    }

    /// Attempts to shift program to a higher priority and enables real-time scheduling.
    ///
    /// # Errors
    /// * This has no effect when not run as root
    pub fn promote_thread_priority(&self, priority: ThreadPriority) -> Result<()> {
        // TODO This should be run in root as it is the only time it will have effect.
        unsafe {
            let result = bindings::piHiPri(priority.0 as i32);
            if result == 0 {
                Ok(())
            } else {
                Err(ErrorKind::WiringPiFail.into())
            }
        }
    }

    /// Registers the provided function to be called for the configured interupt
    /// type on the specified pin.
    ///
    /// The pin number used will depend on how the library was initialized.
    ///
    /// For more information on defining function pointers in Rust refer to the
    /// [FFI Section] of the book. The function takes no parameters and returns
    /// nothing. This function is essentially defined similar to any other
    /// function but is prepended with `extern "C"`.
    ///
    /// [FFI Section]: https://doc.rust-lang.org/book/ffi.html
    pub fn interupt_service_routine(number: i32,
                                    edge_type: InterruptEdgeType,
                                    callback: extern "C" fn()) {
        unsafe {
            bindings::wiringPiISR(number, edge_type as i32, Some(callback));
        }
    }

    // TODO look into how we want to handle the low level multi-threading and if it is necessary
}

// TODO Look into making this a range type of some sort
#[derive(Debug)]
pub struct ThreadPriority(u8);

/// Structure representing a physical GPIO pin on the Raspberyy Pi.
#[derive(Debug)]
pub struct Pin {
    // TODO Create a pin number type that is chosen based
    // on the configuration and ensures that the number is within range.
    number: u8,
    config: WiringPiConfiguration,
}

impl Pin {
    /// Obtains a GPIO pin in the *Input* mode.
    pub fn input(&self) -> InputPin {
        self.set_pin_mode(PinModes::Input);
        InputPin { number: self.number }
    }

    /// Obtains a GPIO pin in the *Output* mode.
    pub fn output(&self) -> OutputPin {
        self.set_pin_mode(PinModes::Output);
        OutputPin { number: self.number }
    }

    /// Obtains a GPIO pin in the *PwmOutput* mode.
    pub fn pwm_output(&self) -> PwmOutputPin {
        self.set_pin_mode(PinModes::PwmOutput);
        PwmOutputPin { number: self.number }
    }

    /// Set the mode of the `Pin` to that specified.
    ///
    /// # Errors
    ///
    /// * If configuration is [`WiringPiConfiguration::Sys`] this
    /// function will have no effect
    ///
    /// [`WiringPiConfiguration::Sys`]: enum.WiringPiConfiguration.html
    fn set_pin_mode(&self, mode: PinModes) {
        // TODO Add logic to ensure that the number of the pin matches the mode
        // attempting to set it to
        if let Ok(_) = self.check_config() {
            unsafe {
                bindings::pinMode(self.number as i32, mode as i32);
            }
        }
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
}

/// GPIO pin for which the mode has been set to [`Input`]
///
/// [`Input`]: constants/enum.PinModes.html#variants
#[derive(Debug)]
pub struct InputPin {
    number: u8,
}

impl InputPin {
    /// Reads the logic level at the pin.
    pub fn digital_read(&self) -> DigitalOut {
        unsafe {
            let result = bindings::digitalRead(self.number as i32);
            result.into()
        }
    }

    /// Returns the value read on the current analog input pin.
    pub fn analog_read(&self) -> i32 {
        unsafe { bindings::analogRead(self.number as i32) }
    }

    /// Set the resistor mode of the `Pin` to that specified.
    ///
    /// # Errors
    ///FIXME
    /// * If configuration is [`WiringPiConfiguration::Sys`] this
    /// function will have no effect
    /// * If current mode of the pin is not [`Input`]
    ///
    ///
    /// [`WiringPiConfiguration::Sys`]: enum.WiringPiConfiguration.html
    /// [`Input`]: constants/enum.PinModes.html#variants
    pub fn set_resistor_mode(&self, mode: ResistorMode) {
        unsafe {
            bindings::pullUpDnControl(self.number as i32, mode as i32);
        }
    }
}

/// GPIO pin for which the mode has been set to [`Output`]
///
/// [`Output`]: constants/enum.PinModes.html#variants
#[derive(Debug)]
pub struct OutputPin {
    number: u8,
}

impl OutputPin {
    /// Writes the given value to the pin.
    pub fn digital_write(&self, value: DigitalOut) {
        unsafe {
            bindings::digitalWrite(self.number as i32, value as i32);
        }
    }

    /// Writes the given analog value to the pin.
    // TODO Need to find out what the limit of value is.
    pub fn analog_write(&self, value: i32) {
        unsafe {
            bindings::analogWrite(self.number as i32, value);
        }
    }
}

/// GPIO pin for which the mode has been set to [`PwmOutput`]
///
/// [`PwmOutput`]: constants/enum.PinModes.html#variants
#[derive(Debug)]
pub struct PwmOutputPin {
    number: u8,
}

impl PwmOutputPin {
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
    pub fn pwm_write(&self, value: u32) {
        // TODO should throw error when not the correct pin
        unsafe {
            bindings::pwmWrite(self.number as i32, value as i32);
        }
    }
}
