use rppal::gpio::Gpio;
use std::error::Error;
use std::fmt;

use std::process;

pub fn register_callback(pin_no: u8) -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(pin_no)?.into_output();
    return Ok(());
}
