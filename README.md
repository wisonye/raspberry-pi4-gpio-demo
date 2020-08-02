# Raspberry Pi4 **GPIO** demo in `Ubuntu-server 20.04`

## `gpio_util`
GPIO util based on `rppal` crate. Encapsulate `rppal` API into easy use interface:


```rust
// Create `InputPin` instance. Return `Error::PinNotAvailable` if fail.
const GPIO_PIN_8: u8 = 8;
let input_pin = GpioUtil::create_input_pin(GPIO_PIN_8).unwrap();
if input_pin.is_low() {}
if input_pin.is_high() {}

// Create `OutputPin` instance. Return `Error::PinNotAvailable` if fail.
const GPIO_PIN_8: u8 = 8;
let output_pin = GpioUtil::create_output_pin(GPIO_PIN_8).unwrap();

// Set pin state
GpioUtil::set_pin_state(output_pin, PinState::High);

// set output pin state and set delay after that
GpioUtil::set_pin_state(output_pin, PinState::High, Some(Duration::from_secs(1)));

```

- How to run `cargo test`?
    
    By default, the 'ubuntu' account can't access the GPIO device via 
`/dev/gpiomem` and `/dev/mem`. So for running the `cargo test` for this 
library, you have to use the command below:

    ```bash
    sudo -E /home/ubuntu/.cargo/bin/cargo test -- --nocapture
    ```

<hr><br>


## How to run the sub project binary

Because the permission issue mentioned above, you can't just run `cargo run`.

You have to use one of the following way:

1. `cargo run`

    ```
    sudo -E /home/ubuntu/.cargo/bin/cargo
    ```

2. Run the compiled binary

    ```
    cargo build
    sudo ./target/debug/xxxx
    ```

3. Run by `cargo watch`

    ```
    sudo -E /home/ubuntu/.cargo/bin/cargo watch -c -x run
    ```

<hr><br>
