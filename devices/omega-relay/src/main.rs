extern crate byteorder;

use std::time::Duration;
use std::thread;

pub mod gpio;

fn main() {
    let port1 = gpio::GPIOPort{number: 1u8};

    port1.set_status(gpio::GPIOPortStatus::Exported)
        .expect("error exporting port");

    port1.set_direction(gpio::GPIOPortDirection::Out)
        .expect("error setting port to out direction");

    let mut port_v = true;

    for _ in 0..10 {
        match port1.set_value(port_v) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("error setting port value to: {}, error: {}",
                          port_v, e);
                break;
            }
        }

        println!("set port to: {}", port_v);

        port_v = !port_v;

        thread::sleep(Duration::from_millis(1000));
    }

    port1.set_status(gpio::GPIOPortStatus::Unexported)
        .expect("error unexporting port");
}
