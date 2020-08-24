use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
// use rppal::gpio::{Result, OutputPin};
// use gpio_util::{GpioUtil, PinState};

// --------------------------------------- Use `wee_alloc` --------------------------------------
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// --------------------------------------- Import from JS ---------------------------------------
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(content: &str);
}


// --------------------------------------- Import from 'js_util.js ' ----------------------------
#[wasm_bindgen(module = "/js_util.js")]
extern "C" {
    fn js_sleep(ms: u32) -> Promise;
}


// --------------------------------------- Thread related ---------------------------------------
#[wasm_bindgen]
pub struct Thread {}

#[wasm_bindgen]
impl Thread {
    /// Call js version promise to wait for particular milliseconds and return as promise
    ///
    /// Here is the [Link](https://rustwasm.github.io/wasm-bindgen/reference/js-promises-and-rust-futures.html)
    /// about how `async fn` return `JS Promise`
    ///
    pub async fn sleep_in_seconds(duration: u32) -> Result<JsValue, JsValue> {
        let temp_promise = js_sleep(duration);
        let temp_future = JsFuture::from(temp_promise);
        let _ = temp_future.await;
        // Ok(JsValue::from_str("Promise result from rust"))
        Ok(JsValue::null())
    }
}


// --------------------------------------- GpioUtil ---------------------------------------------
#[wasm_bindgen]
pub struct GpioUtil {}


// --------------------------------------- SG90ServoMotor ---------------------------------------
#[wasm_bindgen]
pub struct SG90ServoMotor {
    motor_name: String,
}

#[wasm_bindgen]
impl SG90ServoMotor {
    pub fn init_motor() -> SG90ServoMotor {
        SG90ServoMotor {
            motor_name: "SG90 Servo Motor".to_string(),
        }
    }

    pub fn get_motor_name(&self) -> String {
        self.motor_name.clone()
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello {}", name)
}
