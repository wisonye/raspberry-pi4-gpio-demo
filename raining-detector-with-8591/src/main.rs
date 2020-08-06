/// Blinking LED demo
///
/// How to connect the circuit: 
///
/// Pi               8591              LM393(Raining Detector)  Pi
/// ====             ====              ========                ====
/// 3.3V ----------  VCC               VCC      -------------- 3.3V
/// GND  ----------  GND               GND      -------------- GND
/// SDA  ----------  SDA
/// SCL  ----------  SCL
///                  AIN0  ----------- A0
/// GPIO18 --------------------------- D0
/// 
/// You need to run by `sudo` if you see the error below:
///
/// `Error: PermissionDenied("/dev/gpiomem")`

use std::{ thread, time::Duration };
use rppal::gpio::{Result};
use gpio_util::{GpioUtil};
use pcf8591::{PCF8591, Pin};


const GPIO_PIN: u8 = 17;

///
fn main() -> Result<()>  {
    let mut converter = PCF8591::new("/dev/i2c-1", 0x48, 3.3).unwrap();
    let raining_detector_pin = GpioUtil::create_input_pin(GPIO_PIN).unwrap();
     
    let mut status: String = String::new();
    let mut last_raining_flag: bool = false;
    let mut last_raining_analog_value: u8 = 255;

    loop {
        status.clear();

        // `HIGH` means not raining, `LOW` means raining
        let is_raning_flag = raining_detector_pin.is_low();

        // Analog value is a byte (0~255)
        let raining_analog_value = converter.analog_read_byte(Pin::AIN0).unwrap();

        let mut changed = false;
        if last_raining_flag != is_raning_flag || last_raining_analog_value != raining_analog_value { changed = true; }

        // Only update and print out when changed
        if changed == true {
            last_raining_flag = is_raning_flag;
            last_raining_analog_value = raining_analog_value;

            status.push_str(format!("Is it raining? {}", last_raining_flag).as_str());

            status.push_str(format!(", raining analog value: {}", last_raining_analog_value).as_str());

            println!("status: {}", status);
        }

        thread::sleep(Duration::from_millis(10));
    }
}
