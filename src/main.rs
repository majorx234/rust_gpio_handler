use gpio_lib::register_callback;
use std::env::args;
use std::fmt::Error;

fn main() {
    let mut argit = args();
    let gpio_no = argit.nth(2).clone();
    let gpio_no = if let Some(gpio_no) = gpio_no {
        if let Ok(gpio_no) = str::parse::<u8>(&gpio_no) {
            gpio_no
        } else {
            panic!("gpio_no isn't given as u8 value");
        }
    } else {
        panic!("No gpio_no argument given");
    };
    register_callback(gpio_no);
}
