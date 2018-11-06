/*  
use hyper::{Uri};//, Method, Request};

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
    //pub fn register(&self) -> Box<Future<Item=(), Error=String>> {
    pub fn register(&self) -> Result<(), String> {
        // URL
        let mut req_url_str = self.control_server_host.clone();
        req_url_str.push_str("/api/v0/register");

        let req_url: Uri = try!(req_url_str.parse()
            .map_err(|e| format!("error parsing request URL: {}", e)));

        Ok(())
        /*
        // ... Body
        let req_body = json!({
            "physical_id": self.physical_id,
            "default_state": DeviceClientState::Off.to_str(),
        });

        let res = client.post(req_url)
            .body(req_body.to_string())
            .send()
        */
    }
}
*/
