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
    sudo -E /home/ubuntu/.cargo/bin/cargo run
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

## `cargo install` failure solution

The current `ubunut-server 20.04` not include all the rust compilation dependencies. Sometime will fail when
running `cargo install` with the follow error message:

```
error: failed to run custom build command for `openssl-sys v0.9.58`

run pkg_config fail: "`\"pkg-config\" \"--libs\" \"--cflags\" \"openssl\"` did not exit successfully: exit code: 1\n--- stderr\nPackage openssl was not found in the pkg-config search path.\nPerhaps you should add the directory containing `openssl.pc\'\nto the PKG_CONFIG_PATH environment variable\nNo package \'openssl\' found\n"

--- stderr
thread 'main' panicked at '

Could not find directory of OpenSSL installation, and this `-sys` crate cannot
proceed without this knowledge. If OpenSSL is installed and this crate had
trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
compilation process.

Make sure you also have the development packages of openssl installed.
For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.

If you're in a situation where you think the directory *should* be found
automatically, please open a bug at https://github.com/sfackler/rust-openssl
and include information about your system as well as this message.

$HOST = aarch64-unknown-linux-gnu
$TARGET = aarch64-unknown-linux-gnu
openssl-sys = 0.9.58
```

And I've checked that error online, it's not easy to solve. So the workaround is that:

- Install `Docker`.jJW

- Pull `arm64v8/rust` image which is the `Rust` official compilation environment. Then run `cargo install` inside the container.

</br>

```
# Make sure create `~/temp` foler if not exists
mkdir ~/temp

# Run `arm64v8/rust` container to install `wasm-bindgen`
docker run -it --rm --name test -v ~/temp:/app arm64v8/rust

cargo install wasm-bindgen

cp -rvf /usr/local/cargo/bin/wasm* /app

# Exit container
exit
```

Finally, copy the binary files back to `~/.cargo/bin` on the host:

```
cp -rvf ~/temp/wasm* ~/.cargo/bin
```

<hr><br>

## About `rust_wasm_to_nodejs_demo`

This is a demo to show how to build `WASM` for `nodejs` version by `Rust`.

- `npm install` at `rust_wasm_to_nodejs_demo` root folder.

- `rust_wasm_to_nodejs_demo/rust-pi-util/src/lib.rs` is the place for all rust `WASM` code.

- `npm run build-wasm` will generate the `rust_wasm_to_nodejs_demo/rust-pi-util/pkg` folder which includes all WASM related files.

- `npm start`, watch `rust_wasm_to_nodejs_demo/use-rust-util.ts` and run it by `ts-node`

