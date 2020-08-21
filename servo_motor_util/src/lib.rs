// #![allow(warnings)]

/// 1. How `PWM` workd?
///    `PWM` stands for `Pulse-width Modulation`. The simple understanding is that: 
///    Changing the `duty-cycle` to control how much percent of `set to high` (turn on) in a given
///    time for getting the particular working voltage. For example, if your PWM device supply
///    totally `5V`, `0 ~ 100%` duty cycle map to `0 ~ 5V`.
///
/// 2. First, you need to enable the PWM functionality by add the setting below to `/boot/firmware/usercfg.txt` and reboot:
/// dtoverlay=pwm-2chan
///
/// 3. After reboot, you will be able to see `/sys/class/pwm/pwmchip0/` folder there. Otherwise,
///    you will get `NotFound` error when u call `Pwm::with_frequency()`.

use std::{thread, time::Duration};
use rppal::{
    pwm::{Channel, Polarity, Pwm, Result},
};


// ----------------------------------- SG90 Servo Motor ------------------------------------------
const SG90_MOTOR_PERIOD_MS: f64 = 20f64; // Unit in `ms`
const SG90_MOTOR_FREQUENCE: f64 = 50f64; // Unit in `hertz (Hz)`

// For rotate to the position `0`, it needs `1.5 ms puls` which means `1.5 / 20` = 7.5% duty cycle
const SG90_MOTOR_INIT_POSITION_DUTY_CYCLE: f64 = 1.5 / SG90_MOTOR_PERIOD_MS;

// For rotate to the position `-90`, it needs `2.4 ms puls` which means `2.4 / 20` = 12% duty cycle
const SG90_MOTOR_NEGATIVE_90_DEGREE_DUTY_CYCLE: f64 = 2.4 / SG90_MOTOR_PERIOD_MS; // 

// For rotate to the position `90`, it needs `0.5 ms puls` which means `0.5 / 20` = 2.5% duty cycle
const SG90_MOTOR_POSITIVE_90_DEGREE_DUTY_CYCLE: f64 = 0.5 / SG90_MOTOR_PERIOD_MS;

///
pub enum SG90ServoMotorInitPosition {
   Init,
   Negative90Degree,
   Positive90Degree
}

///
pub struct SG90ServoMotor {
}

///
impl SG90ServoMotor {

    /// By default, use `Channel::Pwm0` which is the `GPIO18` on the board
    /// If `use_pwm1` set to `true`, then use `Channel::Pwm1` which is then
    /// `GPIO19` on the board.
    pub fn get_motor_with_init_position(init_position: SG90ServoMotorInitPosition, use_pwm1: bool) -> Result<Pwm> {
        let mut sg90_motor = Pwm::with_frequency(
            if use_pwm1 { Channel::Pwm1 } else { Channel::Pwm0 },
            SG90_MOTOR_FREQUENCE,
            SG90_MOTOR_INIT_POSITION_DUTY_CYCLE,
            Polarity::Normal,
            true
        );

        if sg90_motor.is_err() { return sg90_motor; }

        // Give enough motor enough time to rotate to the init position.
        thread::sleep(Duration::from_millis(250));

        match init_position {
            SG90ServoMotorInitPosition::Init => {},
            SG90ServoMotorInitPosition::Negative90Degree => { let _ = sg90_motor.as_mut().unwrap().set_duty_cycle(SG90_MOTOR_NEGATIVE_90_DEGREE_DUTY_CYCLE); }
            SG90ServoMotorInitPosition::Positive90Degree => { let _ = sg90_motor.as_mut().unwrap().set_duty_cycle(SG90_MOTOR_POSITIVE_90_DEGREE_DUTY_CYCLE); }
        }
        // }

        sg90_motor
    }

    ///
    pub fn rotate_to_init_position(motor: &mut Result<Pwm>) {
        let _ = motor.as_mut().unwrap().set_duty_cycle(SG90_MOTOR_INIT_POSITION_DUTY_CYCLE); 
    }

    ///
    pub fn rotate_to_negative_position(motor: &mut Result<Pwm>) {
        let _ = motor.as_mut().unwrap().set_duty_cycle(SG90_MOTOR_NEGATIVE_90_DEGREE_DUTY_CYCLE); 
    }

    ///
    pub fn rotate_to_positive_position(motor: &mut Result<Pwm>) {
        let _ = motor.as_mut().unwrap().set_duty_cycle(SG90_MOTOR_POSITIVE_90_DEGREE_DUTY_CYCLE); 
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
