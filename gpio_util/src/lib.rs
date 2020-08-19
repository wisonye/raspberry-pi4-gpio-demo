#![allow(warnings)]

// GPIO lib based on `rppal` crate
//
// Pay attention:
//     
//     By default, the 'ubuntu' account can't access the GPIO device via 
// `/dev/gpiomem` and `/dev/mem`. So for running the `cargo test` for this 
// library, you have to use the command below:
//
// ```bash
// sudo -E /home/ubuntu/.cargo/bin/cargo test -- --nocapture
// ```

use rppal::gpio::{ Gpio, InputPin, OutputPin, Level, Result };
use std::{ thread, time::Duration };

///
#[derive(Debug, Clone)]
pub enum  PinState {
    Low = 0,
    High = 1,
}

///
#[derive(Debug)]
pub struct GpioUtil {}

impl GpioUtil {
    
    /// Create `InputPin` instance. Return `Error::PinNotAvailable` if fail.
    pub fn create_input_pin(gpio_pin_number: u8) -> Result<InputPin> {
        // `Gpio::new` get back a unconfigured `Gpio` instance
        // `Gpio::get` get back the `BCM GPIO` `Pin` instance. The pin number is `BCM GPIO` pin
        // number!!!
        Ok(Gpio::new()?.get(gpio_pin_number)?.into_input())
    }

    /// Create `OutputPin` instance. Return `Error::PinNotAvailable` if fail.
    pub fn create_output_pin(gpio_pin_number: u8) -> Result<OutputPin> {
        // `Gpio::new` get back a unconfigured `Gpio` instance
        // `Gpio::get` get back the `BCM GPIO` `Pin` instance. The pin number is `BCM GPIO` pin
        // number!!!
        Ok(Gpio::new()?.get(gpio_pin_number)?.into_output())
    }

    /// Set pin state
    pub fn set_pin_state(pin: &mut OutputPin, init_state: PinState, delay_after_reset: Option<Duration>) {
        match init_state {
            PinState::Low => pin.set_low(),
            PinState::High => pin.set_high()
        }

        if delay_after_reset.is_some() {
            thread::sleep(delay_after_reset.unwrap())
        }
    }

    /// Keep reading state from button pin, only return when button pressed
    /// and then released (`HIGH->LOW->HIGH`) happens.
    pub fn block_until_button_pressed(button_pin: &InputPin) {
        let mut init_high = false;
        let mut pressed = false;
        let mut released = false;
        let button_read_period = Duration::from_millis(10);
        loop {
            let temp_button_state = button_pin.read();

            // init high
            if temp_button_state == Level::High && init_high == false { init_high = true; }
            // Detect pressed
            else if temp_button_state == Level::Low && init_high == true { pressed = true; }
            // Detect released after pressing
            else if temp_button_state == Level::High && init_high == true && pressed == true {
                released = true;
                return;
            }

            thread::sleep(button_read_period)
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_input_pin_should_fail_without_sudo_permission() {
        let input_pin = GpioUtil::create_input_pin(8);

        // It should print out `Error(PermissionDenied("/dev/gpiomem"))`
        // if you run without `sudo`!!!
        // println!("input_pin {:?}", input_pin);

        assert_eq!(input_pin.is_ok(), true);
    }

    #[test]
    fn set_pin_state_should_work() {
        let mut output_pin = GpioUtil::create_output_pin(25).unwrap();
        println!("output_pin {:#?}", output_pin);
        assert_eq!(output_pin.is_set_low(), true);

        output_pin.set_high();
        assert_eq!(output_pin.is_set_high(), true);

        output_pin.set_low();
        assert_eq!(output_pin.is_set_low(), true);
    }

}
