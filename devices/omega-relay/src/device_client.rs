use hyper::{Client, Request, Method, header::CONTENT_TYPE};
use futures::future::{Future, ok, err};

/// Communicates with the control server
pub struct DeviceClient {
    /// Host location of control server
    pub control_server_host: String,

    /// The physical id to send to the server
    pub physical_id: String,
}

/// Is the state of the device
#[derive(Debug)]
pub enum DeviceClientState {
    /// Indicates that the relay is switched on
    On,

    /// Indicates that the relay is switched off
    Off,
}

impl DeviceClientState {
    /// Returns a DeviceClientState enum based on a string value
    pub fn from_string(value: &str) -> Result<DeviceClientState, String> {
        if value == "1" {
            Ok(DeviceClientState::On)
        } else if value == "0" {
            Ok(DeviceClientState::Off)
        } else {
            Err(format!("invalid value: {}", value))
        }
    }

    /// Returns a str based on a DeviceClientState enum value
    pub fn to_str(&self) -> &str {
        match self {
            DeviceClientState::On => "1",
            DeviceClientState::Off => "0"
        }
    }
}

impl DeviceClient {
    /// Register device with the control server
    pub fn register(&self) -> Box<Future<Item=(), Error=String>> {
        // Prepare parameters
        let client = Client::new();

        // ... URL
        let mut req_url = self.control_server_host.clone();
        req_url.push_str("/api/v0/register");

        /*
        let req_url = try_future!(req_url.parse::<Uri>()
            .map_err(|e| format!("error parsing request url: {}", e)));
        */

        // ... Body
        let req_body = json!({
            "physical_id": self.physical_id,
            "default_state": DeviceClientState::Off.to_str(),
        });

        // Build request
        let req = Request::builder()
            .method(Method::POST)
            .uri(req_url)
            .header(CONTENT_TYPE, "application/json")
            .body(req_body.to_string());

        let req = match req.unwrap() {
            Ok(r) => r,
            Err(e) => Box::new(err(format!("error building request: {}", e)))
        }

        // Make request
        client.request(req)
            .and_then(|res| {
                println!("register response status: {}", res.status());

                ok(())
            })
            .map_err(|e| format!("error making request: {}", e))
    }
}
