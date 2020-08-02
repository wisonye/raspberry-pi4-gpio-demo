/// Blinking LED demo
///
/// You need to run by `sudo` if you see the error below:
///
/// `Error: PermissionDenied("/dev/gpiomem")`

use std::{ time::Duration };
use rppal::gpio::{Result};
use gpio_util::{GpioUtil, PinState};


const GPIO_PIN_LED: u8 = 18;

///
fn main() -> Result<()>  {
    let mut led = GpioUtil::create_output_pin(GPIO_PIN_LED).unwrap();
    
    // By default, pins are set to their original state when be dropped, 
    // you can use the method below to disable this feature:
    //
    // `InputPin::set_reset_on_drop(false)`
    // `OutputPin::set_reset_on_drop(false)`
    //
    // For preventing `set_reset_on_drop` is being set, we better to reset to `LOW` right now.
    GpioUtil::set_pin_state(&mut led, PinState::Low, Some(Duration::from_millis(100)));
     
    loop {
        println!("---> LED is on");
        GpioUtil::set_pin_state(&mut led, PinState::High, Some(Duration::from_secs(1)));

        println!("---> LED is off");
        GpioUtil::set_pin_state(&mut led, PinState::Low, Some(Duration::from_secs(1)));
    }

    // Ok(())
}
