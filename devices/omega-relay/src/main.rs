#[macro_use] extern crate try_future;
#[macro_use] extern crate serde_json;
extern crate hyper;
extern crate futures;
extern crate tokio;
extern crate tokio_core;

pub mod gpio;
pub mod device_client;

use hyper::Client;
use hyper::rt;
use futures::Future;
use futures::future::{ok};

use tokio_core::reactor::Core;

use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct FutureError {}

impl fmt::Display for FutureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FutureError.Display") 
    }
}

impl Error for FutureError {
    fn description(&self) -> &str {
        "FutureError.description"
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn foo() -> impl Future<Item=u32, Error=FutureError> {
    ok(100)
}

fn main() {
    // Setup GPIO port
    let port1 = gpio::GPIOPort{number: 1u8};

    port1.set_status(gpio::GPIOPortStatus::Exported)
        .expect("error exporting port");

    port1.set_direction(gpio::GPIOPortDirection::Out)
        .expect("error setting port to out direction");

    println!("Setup GPIO");

    // Setup device client
    let hyper_client = Client::new();
    let device_client = device_client::DeviceClient::new(
        String::from("http://192.168.1.106:5000"),
        String::from("omega-relay"),
        hyper_client);

    let mut reactor = Core::new().unwrap();

    reactor.run(foo()).unwrap();
    //rt::run(device_client.register())
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
