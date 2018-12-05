use hyper::{Uri, Method, Request, Body}; 
use hyper::rt::Future;
use hyper::client::{Client, HttpConnector};

/// Communicates with the control server
pub struct DeviceClient {
    /// Hyper client
    hyper_client: Client<HttpConnector>,

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
    /// Create new DeviceClient
    ///
    /// # Arguments
    ///
    /// * `control_server_host`
    /// * `physical_id`
    /// * `hyper_client`
    pub fn new(control_server_host: String, physical_id: String,
               hyper_client: Client<HttpConnector>) -> DeviceClient {
        return DeviceClient{
            control_server_host: control_server_host,
            physical_id: physical_id,
            hyper_client: hyper_client,
        }
    }

    /// Register device with the control server
    pub fn register(&self) -> Future<Item=(), Error=String> {
        // URL
        let mut req_url_str = self.control_server_host.clone();
        req_url_str.push_str("/api/v0/register");

        let req_url: Uri = try!(req_url_str.parse()
            .map_err(|e| format!("error parsing request URL: {}", e)));

        // ... Body
        let req_body = json!({
            "physical_id": self.physical_id,
            "default_state": DeviceClientState::Off.to_str(),
        });

        let mut req = Request::new(Body::from(req_body.to_string()));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = req_url.clone();
        
        self.hyper_client.request(req)
            .map_err(|e| format!("error making API request: {}", e))
            .and_then(|res| {
                println!("Status: {}", res.status());

                res.into_body().to_string()
            })
    }
}
