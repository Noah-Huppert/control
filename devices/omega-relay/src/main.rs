#[macro_use] extern crate serde_json;
extern crate hyper;

pub mod gpio;
pub mod device_client;

use hyper::rt;
use hyper::Client;

fn main() {
    // Setup GPIO port
    let port1 = gpio::GPIOPort{number: 1u8};

    port1.set_status(gpio::GPIOPortStatus::Exported)
        .expect("error exporting port");

    port1.set_direction(gpio::GPIOPortDirection::Out)
        .expect("error setting port to out direction");

    println!("Setup GPIO");

    // Setup device client
    rt::run(rt::lazy(|| {
        let hyper_client = Client::new();
        let device_client = device_client::DeviceClient::new(
            String::from("http://192.168.1.106:5000"),
            String::from("omega-relay"), hyper_client);

        device_client.register()
    }));
        /*
    let client = device_client::DeviceClient{
        control_server_host: String::from("http://192.168.1.106:5000"),
        physical_id: String::from("omega-relay"),
    };

    //let _ = client.register().unwrap();
        .map_err(|e| format!("error registering device: {}", e))
        .and_then(|_| {
            // Cleanup GPIO port
            port1.set_status(gpio::GPIOPortStatus::Unexported)
                .expect("error unexporting port");

            println!("Cleaned up GPIO");

            ok(())
        });
        */

    //hyper::rt::run(f) 
}
