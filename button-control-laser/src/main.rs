/// Use button to control laser
///
/// How to connect the circuit: 
///
/// Pi               Button   Laser Emitter
/// ====             ======   =============
/// 5V     ----------  VCC 
/// GND    ----------  GND 
/// GPIO17 ----------  SIG
///
/// GPIO27 -----------------  SIG
/// 5V     -----------------  VCC 
/// GND    -----------------  GND 
///
///
/// You need to run by `sudo` if you see the error below:
///
/// `Error: PermissionDenied("/dev/gpiomem")`
///

use std::{thread, time::Duration};
use rppal::gpio::{Result};
use gpio_util::{GpioUtil, PinState};


const GPIO_BUTTON_SIGNAL_PIN: u8 = 17;
const GPIO_LASER_SIGNAL_PIN: u8 = 27;

///
fn main() -> Result<()>  {
    let button_signal = GpioUtil::create_input_pin(GPIO_BUTTON_SIGNAL_PIN).unwrap();
    let mut laser_signal = GpioUtil::create_output_pin(GPIO_LASER_SIGNAL_PIN).unwrap();
    
    // For preventing `set_reset_on_drop` is being set, we better to reset to `Low` right now.
    GpioUtil::set_pin_state(&mut laser_signal, PinState::Low, Some(Duration::from_millis(100)));
     
    let mut laser_is_on = false;
    let loop_duration = Duration::from_millis(100);

    loop {
        GpioUtil::block_until_button_pressed(&button_signal);

        laser_is_on = !laser_is_on;
        println!("laser_is_on: {}", laser_is_on);

        let new_pin_state = if laser_is_on { PinState::High } else { PinState::Low };
        GpioUtil::set_pin_state(&mut laser_signal, new_pin_state, None);

        thread::sleep(loop_duration);
    }
}
