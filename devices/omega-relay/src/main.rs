#[macro_use] extern crate serde_json;
#[macro_use] extern crate try_future;
extern crate hyper;
extern crate futures;

pub mod gpio;
pub mod device_client;

use hyper::rt::run;
use futures::future::{Future, ok};

fn main() {
    // Setup GPIO port
    let port1 = gpio::GPIOPort{number: 1u8};

    port1.set_status(gpio::GPIOPortStatus::Exported)
        .expect("error exporting port");

    port1.set_direction(gpio::GPIOPortDirection::Out)
        .expect("error setting port to out direction");

    println!("Setup GPIO");

    // Setup device client
    let client = device_client::DeviceClient{
        control_server_host: String::from("http://192.168.1.106:5000"),
        physical_id: String::from("omega-relay"),
    };

    let f = client.register()
        .map_err(|e| format!("error registering device: {}", e))
        .and_then(|_| {
            // Cleanup GPIO port
            port1.set_status(gpio::GPIOPortStatus::Unexported)
                .expect("error unexporting port");

            println!("Cleaned up GPIO");

            ok(())
        });

    hyper::rt::run(f) 
}
