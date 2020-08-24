import { greet, SG90ServoMotor, Thread } from './rust-pi-util/pkg/rust_pi_util'

const main = async () => {
    const my_name = `Wison Ye`

    console.log(`greet result: ${greet(my_name)}`)

    try {
        const temp_result = await Thread.sleep(2000n);
        const motor = SG90ServoMotor.init_motor()
        console.log(`motor name: ${motor.get_motor_name()}`)
    } catch (error) {
        console.log(`Error happen: `, error)
    }

}

main()
