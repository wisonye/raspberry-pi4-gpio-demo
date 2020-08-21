/// Use button to control laser
///
/// 1. First, you need to enable the PWM functionality by add the setting below to `/boot/firmware/usercfg.txt` and reboot:
/// dtoverlay=pwm-2chan
///
/// 2. After reboot, you will be able to see `/sys/class/pwm/pwmchip0/` folder there. Otherwise,
///    you will get `NotFound` error when u call `Pwm::with_frequency()`.
///
///
/// How to connect the circuit: 
///
/// Pi               Button   SG90 Motor
/// ====             ======   =============
/// 5V     ----------  VCC 
/// GND    ----------  GND 
/// GPIO17 ----------  SIG
///
/// GPIO18 -----------------  Pwm(Orange)
/// 5V     -----------------  VCC(Red) 
/// GND    -----------------  GND(Brown) 
///
///
/// You need to run by `sudo` if you see the error below:
///
/// `Error: PermissionDenied("/dev/gpiomem")`
///

use rppal::{
    gpio::{Result}
};
use gpio_util::{GpioUtil};
use servo_motor_util::{SG90ServoMotor, SG90ServoMotorInitPosition};

const GPIO_BUTTON_SIGNAL_PIN: u8 = 17;


///
fn main() -> Result<()> {
    let mut sg90_motor = SG90ServoMotor::get_motor_with_init_position(SG90ServoMotorInitPosition::Init, false);
    match sg90_motor  {
        Ok(ref pwm_value) => {
            println!("SG90 motor set to init position, {:?}", pwm_value);
        }
        // Exit if open PWM fail.
        Err(error) => {
            println!("SG90 motor init fail: {:?}", error);
            return Ok(());
        }
    }

    let button_signal = GpioUtil::create_input_pin(GPIO_BUTTON_SIGNAL_PIN).unwrap();

    let mut open_gate = false;

    loop {
        GpioUtil::block_until_button_pressed(&button_signal);

        if !open_gate { let _  = SG90ServoMotor::rotate_to_positive_position(&mut sg90_motor); }
        else { let _  = SG90ServoMotor::rotate_to_init_position(&mut sg90_motor); }

        open_gate = !open_gate;
    }
}
