extern crate wiringpi_rs;

use wiringpi_rs::{WiringPi, WiringPiConfiguration};
use wiringpi_rs::constants::*;

fn main() {
    let pi = WiringPi::new(WiringPiConfiguration::Def);
    let pin = pi.pin(15).output();

    loop {
        pin.digital_write(DigitalOut::High);
        pi.delay(500);
        pin.digital_write(DigitalOut::Low);
        pi.delay(500);
    }
}
