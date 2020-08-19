/// Simulate high-tech laser detecting entry security system demo:
///
/// 1. Tap on button to toggle the security system enable or disable.
///
/// 2. When then system is enabled, laser emitter and laser light detector are turned on.
///    So if somebody walks through the laser line which will block the light reaching the
///    light detector, alarm will be triggered, alarm buzzer will make noise.
///
///    When the `alarm_trigger_signal` is `high`, it means the security camera will be turned
///    on recording and send the alarm signal to police station.
///
/// How to connect the circuit: 
///
/// Pi               Button   Laser  System Status Alarm    Buzzer      Light Detector  8951
/// ====             ======   ====== ============= ======   ======      ==============  ====
/// 5V     ----------  VCC
/// GND    ----------  GND 
/// GPIO17 ----------  SIG
///
/// GPIO27 -----------------  SIG
/// 5V     -----------------  VCC 
/// GND    -----------------  GND 
///
/// GPIO19 ------------------------------------------------ SIG(LOW)
///
/// GPIO20 --------------------------------------- SIG ---- VCC
/// GND    --------------------------------------- GND ---- GND
///
/// GPIO21 ------------------------- SIG ------------------------------ VCC
/// GND    ------------------------- GND ------------------------------ GND
///                                                                     AO  ----------- AIN0
/// 3.3V   ---------------------------------------------------------------------------- VCC
/// GND    ---------------------------------------------------------------------------- GND
/// SCL    ---------------------------------------------------------------------------- SCL
/// SDA    ---------------------------------------------------------------------------- SDA
///
/// You need to run by `sudo` if you see the error below:
///
/// `Error: PermissionDenied("/dev/gpiomem")`
///

use std::{
    sync::mpsc::{channel, Sender},
    thread, 
    time::Duration
};
use rppal::gpio::{Result, OutputPin};
use gpio_util::{GpioUtil, PinState};
use pcf8591::{PCF8591, Pin};


const GPIO_BUTTON_SIGNAL_PIN: u8 = 17;
const GPIO_ALARM_BUZZER_PIN: u8 = 19;
const GPIO_ALARM_TRIGGER_PIN: u8 = 20;
const GPIO_SYSTEM_STATUS_INDICATOR_PIN: u8 = 21;
const GPIO_LASER_SIGNAL_PIN: u8 = 27;


///
enum SystemEventType {
    ButtonPressed,
    AlarmHappened
}


/// Button signal thread, it will send `SystemEventType::ButtonPressed` to channel when button
/// pressed.
fn start_button_signal_thread(event_sender: Sender<SystemEventType>) -> thread::JoinHandle<()> {
    let button_signal = GpioUtil::create_input_pin(GPIO_BUTTON_SIGNAL_PIN).unwrap();
    let thread_loop_duration = Duration::from_millis(50);

    thread::spawn(move || {
        loop {
            GpioUtil::block_until_button_pressed(&button_signal);
            let _ = event_sender.send(SystemEventType::ButtonPressed);
            thread::sleep(thread_loop_duration);
        }
    })
}
 

/// This thread will keep reading the analog value from `PCF8591 AIN0` pin. When something is blocking
/// the laser light towards to the light detector, analog value will become `255`, then fire an
/// alarm to the channel.
fn start_alarm_checking_thread(event_sender: Sender<SystemEventType>) -> thread::JoinHandle<()> {
    // - `path`: device slave path (0x48 per default)
    // - `address`: has to be defined as per Table 5.
    // - `v_ref`: is the board voltage (e.g. typically 3.3V on raspberry pi)
    let mut converter = PCF8591::new("/dev/i2c-1", 0x48, 3.3).unwrap();
    
    let thread_loop_duration = Duration::from_millis(50);
    let mut laser_light_is_blocking = false;

    thread::spawn(move || {
        loop {
            // Analog value is a byte (0~255)
            let analog_value = converter.analog_read_byte(Pin::AIN0).unwrap();

            if !laser_light_is_blocking && analog_value > 20 {
                laser_light_is_blocking = true;
                let _ = event_sender.send(SystemEventType::AlarmHappened);
            } else if analog_value < 255 {
                laser_light_is_blocking = false;
            }

            // println!("laser_light_is_blocking: {}, analog_value: {}", laser_light_is_blocking, analog_value);
            // println!("analog_value: {}", analog_value);

            thread::sleep(thread_loop_duration);
        }
    })
}

///
fn toggle_security_system(
    already_enbaled: bool, 
    laser_signal: &mut OutputPin, 
    system_status_indicator_signal: &mut OutputPin
    ) {

    let enable = !already_enbaled;
    let status_desc = if enable {
        "\n[ Laser detecting entry security system is Enabled ]\n"
    } else {
        "\n[ Laser detecting entry security system is Disabled ]\n"
    };
    
    println!("{}", status_desc);

    let laser_pin_state = if enable { PinState::High } else { PinState::Low };
    let system_status_indicator_state = laser_pin_state.clone();
    GpioUtil::set_pin_state(laser_signal, laser_pin_state, None);
    GpioUtil::set_pin_state(system_status_indicator_signal, system_status_indicator_state, None);
}


///
fn main() -> Result<()>  {
    // Create event bus
    let (event_sender, event_bus) = channel();


    let mut laser_signal = GpioUtil::create_output_pin(GPIO_LASER_SIGNAL_PIN).unwrap();
    let mut system_status_indicator_signal = GpioUtil::create_output_pin(GPIO_SYSTEM_STATUS_INDICATOR_PIN).unwrap();
    let mut alarm_trigger_signal = GpioUtil::create_output_pin(GPIO_ALARM_TRIGGER_PIN).unwrap();
    let mut alarm_buzzer_signal = GpioUtil::create_output_pin(GPIO_ALARM_BUZZER_PIN).unwrap();
    
    // Make sure disable the system by default
    toggle_security_system(
        true, 
        &mut laser_signal,
        &mut system_status_indicator_signal
    );

    println!("\n<<< Laser detecting entry security system is Ready >>>\n");
    println!("Please tap on the button to toggle the system status.\n");

     
    let mut security_system_is_already_enabled = false;
    let main_loop_duration = Duration::from_millis(100);

    start_button_signal_thread(event_sender.clone());
    start_alarm_checking_thread(event_sender.clone());

    loop {
        match event_bus.try_recv() {
            Ok(SystemEventType::ButtonPressed) => {
                toggle_security_system(
                    security_system_is_already_enabled, 
                    &mut laser_signal,
                    &mut system_status_indicator_signal
                    );

                security_system_is_already_enabled = !security_system_is_already_enabled;
            },
            Ok(SystemEventType::AlarmHappened) => {
                println!("Alarm: Unauthorized entry detected......", );
                GpioUtil::set_pin_state(&mut alarm_trigger_signal, PinState::High, None);
                GpioUtil::set_pin_state(&mut alarm_buzzer_signal, PinState::Low, None);
            }
            _ => {
                GpioUtil::set_pin_state(&mut alarm_trigger_signal, PinState::Low, None);
                GpioUtil::set_pin_state(&mut alarm_buzzer_signal, PinState::High, None);
            }
        }

        thread::sleep(main_loop_duration);
    }
}
