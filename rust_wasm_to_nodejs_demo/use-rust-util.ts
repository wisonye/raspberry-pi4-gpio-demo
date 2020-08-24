import { greet, SG90ServoMotor, Thread } from './rust-pi-util/pkg/rust_pi_util'

const main = async () => {
    const my_name = `Wison Ye`

    console.log(`greet result: ${greet(my_name)}`)

    // const promise_result = await Thread.sleep(2000n);
    const promise_result = await Thread.sleep_in_seconds(1000);
    console.log(`promise_result >> : ${promise_result}`)

    const motor = SG90ServoMotor.init_motor()
    console.log(`motor name: ${motor.get_motor_name()}`)
}



main()
