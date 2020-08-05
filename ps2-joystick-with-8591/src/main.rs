/// Use PCF8591 (AD/DA converter) to control PS joystick
///
/// How to connect the circuit: 
///
/// Pi               8591              Joystick       Pi
/// ====             ====              ========       ====
/// 3.3V ----------  VCC               +5V      ----- 3.3V
/// GND  ----------  GND               GND      ----- GND
/// SDA  ----------  SDA
/// SCL  ----------  SCL
///                  AIN0  ----------- VRX
///                  AIN1  ----------- VRY
///                  AIN2  ----------- SW
///
/// You need to run by `sudo` if you see the error below:
///
/// `Error: PermissionDenied("/dev/gpiomem")`
///

use std::{time::Duration, thread, error::Error};
use pcf8591::{PCF8591, Pin};

fn main() -> Result<(), Box<dyn Error>> {
    let mut converter = PCF8591::new("/dev/i2c-1", 0x48, 3.3).unwrap();

    loop {
        // Analog value is a byte (0~255)
        let x = converter.analog_read_byte(Pin::AIN0).unwrap();
        let y = converter.analog_read_byte(Pin::AIN1).unwrap();
        let z = converter.analog_read_byte(Pin::AIN2).unwrap();
        // println!("x: {}", x);
        // println!("y: {}", y);
        // println!("z: {}", z);
        //

        let mut status: String = String::new();

        if x == 0 { status.push_str("Left,"); }
        else if x == 255 { status.push_str("Right,"); }

        if y == 0 { status.push_str("Up,"); }
        else if y == 255 { status.push_str("Down,"); }

        if z == 0 { status.push_str("Button pressed"); }

        if status.len() > 0 { println!("Joystick status: {}", status);}

        thread::sleep(Duration::from_millis(10));
    }
}
