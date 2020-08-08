/// Use PCF8591 (AD/DA converter) and potentiometer to control LED dimming
///
/// `Potentiomenter` uses to generate the changeable analog value which representing
/// the min ~ max resistor value.
///
/// `PCF8591` can read 3 analog values via `AIN0 ~ ANI3`, also be able to write back
/// an analog value to `AOUT`. The `AOUT` output is a voltage after converted with
/// the analog value. (0 ~ 255 --- map to --> 0 ~ max voltage).
///
/// How to connect the circuit: 
///
/// Pi               8591              Potentiomenter       Pi       LED
/// ====             ====              ==============       ====     ===
/// 3.3V ----------  VCC               VCC            ----- 3.3V
/// GND  ----------  GND               GND            ----- GND  --- GND
/// SDA  ----------  SDA
/// SCL  ----------  SCL
///                  AIN0  ----------- OUT
///                  AOUT  ----------------------------------------- VCC
///
/// You need to run by `sudo` if you see the error below:
///
/// `Error: PermissionDenied("/dev/gpiomem")`
///

use std::{time::Duration, thread, error::Error};
use pcf8591::{PCF8591, Pin};

///
fn main() -> Result<(), Box<dyn Error>> {
    // - `path`: device slave path (0x48 per default)
    // - `address`: has to be defined as per Table 5.
    // - `v_ref`: is the board voltage (e.g. typically 3.3V on raspberry pi)
    let mut converter = PCF8591::new("/dev/i2c-1", 0x48, 3.3).unwrap();

    loop {
        // [ Handle with analog value (0 ~ 255)
        // Analog value is a byte (0~255)
        // let analog_value = converter.analog_read_byte(Pin::AIN0).unwrap();
        // println!("analog_value: {}", analog_value);

        // Write back the voltage (related to the analog_value) to `AOUT`
        // let _  = converter.analog_write_byte(analog_value);


        // [ Handle with voltage after converted by analog value (0 ~ `v_ref`)
        // Read back the voltage which after converted via analog_value
        let voltage_from_analog_value = converter.analog_read(Pin::AIN0).unwrap();
        println!("voltage_from_analog_value: {:.2}v", voltage_from_analog_value);

        let _  = converter.analog_write(voltage_from_analog_value);

        thread::sleep(Duration::from_millis(10));
    }
}
