// this is what coding without thinking looks like. Back to the drawing board.

use gpio::{GpioIn};
use std::{thread, time};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let mut gpio0 = gpio::sysfs::SysFsGpioInput::open(0).unwrap();
    println!("hello hall!");
    loop {
        let mut instant_speed = 0; 
        let mut sensor_state = gpio0.read_value().unwrap();
        println!("{:?}", sensor_state);
        // print_type_of(&sensor_state);
        if sensor_state == gpio::GpioValue::High {
            let start = time::Instant::now();
            println!("record time");
            println!("{:?} wait to drop low", start.elapsed());
            while sensor_state == gpio::GpioValue::High {
                sensor_state = gpio0.read_value().unwrap();
                thread::sleep(time::Duration::from_millis(1));
            }

            println!("{:?} wait to go high again", start.elapsed());
            while sensor_state == gpio::GpioValue::Low {
                sensor_state = gpio0.read_value().unwrap();
                thread::sleep(time::Duration::from_millis(1));
            }
            let duration = start.elapsed();
            println!("{:?}", duration)
        }

    }

}