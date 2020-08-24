use wasm_bindgen::prelude::*;
use std::{thread, time::Duration};
// use rppal::gpio::{Result, OutputPin};
// use gpio_util::{GpioUtil, PinState};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// --------------------------------------- Thread related ---------------------------------------
#[wasm_bindgen]
pub struct Thread { }


#[wasm_bindgen]
impl Thread {
    /// Puts the current thread to sleep for at least the specified amount of time.
    /// `duration` unit is milliseconds
    pub async fn sleep(duration: u64) -> Result<JsValue, JsValue> {
        // let temp_result = thread::sleep(Duration::from_millis(duration));
        Ok("Promise success".to_string())
    }
}

// --------------------------------------- GpioUtil ---------------------------------------------
#[wasm_bindgen]
pub struct GpioUtil {
    
}


// --------------------------------------- SG90ServoMotor ---------------------------------------
#[wasm_bindgen]
pub struct SG90ServoMotor {
   motor_name: String 
}

#[wasm_bindgen]
impl SG90ServoMotor {
   pub fn init_motor() -> SG90ServoMotor {
       SG90ServoMotor { motor_name: "SG90 Servo Motor".to_string() }
   }

   pub fn get_motor_name(&self) -> String {
       self.motor_name.clone()
   }
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello {}", name)
}
