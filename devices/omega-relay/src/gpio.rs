use byteorder::WriteBytesExt;
use std::path::Path;
use std::io;
use std::io::Write;
use std::fs::File;
use std::mem;

/// Operating system path where GPIO device files are located
const GPIO_PORT_PATH: &str = "/sys/class/gpio";

/// Interface to the OS system which controls general input output ports
///
/// # GPIO OS System Overview
/// GPIO ports are controlled via files in the `GPIO_PORT_PATH` directory.
///
/// Each exported GPIO port gets its own directory named in format:
///
/// ```
/// gpio<number>
/// ```
///
/// Where `<number>` is a unsigned integer used to identify the GPIO port.
///
/// ## Port Initialization
/// Before a GPIO port can be controlled by the OS it must be exported.
///
/// To export a GPIO port write the port number to the `/export` file.  
///
/// To unexport a GPIO port write the port number to the `/unexport` file.
///
/// ## Control Files
/// Inside each GPIO port directory multiple files exist which control
/// the port.
///
/// ### Direction
/// **File**: `/direction`  
/// **Valid values**: `in` or `out`  
/// 
/// If the port is outputting or receiving signals.  
///
/// Read this file to determine the direction.  
/// Write to this file to set the direction
///
/// ### Value
/// **File**: `/value`  
/// **Valid values**: `0` or `1`  
///
/// If the port is on or off.
///
///
/// Read this file to determine the value.  
/// Write to this file to set the value
pub struct GPIOPort {
    /// Identifying port number
    pub number: u8,
}

/// Indicates the status of a GPIOPort with the OS
#[derive(Debug)]
pub enum GPIOPortStatus {
    /// Indicates that the GPIO port is being controlled by the OS
    Exported,

    /// Indicates that the GPIO port is not yet being controlled by the OS
    Unexported
}

fn u8_to_bytes(int: u8) -> [u8; 1] {
    let mut bytes = [0u8; mem::size_of::<u8>()];
    match bytes.as_mut().write_u8(int) {
        Ok(_v) => (),
        Err(e) => panic!("error writing uint to byte array: {}", e)
    }

    bytes
}

impl GPIOPort {
    /// Returns the directory where a specific GPIO ports device files 
    /// are located
    fn get_path(&self) -> String {
        return format!("{}/gpio{}", GPIO_PORT_PATH, self.number);
    }

    /// Returns the status of a GPIO port with the OS
    pub fn get_status(&self) -> GPIOPortStatus {
        if Path::new(&self.get_path()).exists() {
            GPIOPortStatus::Exported
        } else {
            GPIOPortStatus::Unexported
        }
    }

    /// Sets GPIO port status with OS
    pub fn set_status(&self, status: GPIOPortStatus) -> Result<(), io::Error> {
        // Prepare to write to the /export or /unexport file based on status
        let mut status_f_path = GPIO_PORT_PATH.to_owned();

        match status {
            GPIOPortStatus::Exported => {
                status_f_path.push_str("/export");
            },
            GPIOPortStatus::Unexported => {
                status_f_path.push_str("/unexport");
            }
        }

        panic!("debug: {:?}", status_f_path);

        // Write to file
        let mut status_f = match File::create(status_f_path) {
            Ok(f) => f,
            Err(e) => panic!("error opening status file: {}", e)
        };

        let write_v = u8_to_bytes(self.number);
     
        match status_f.write_all(&write_v) {
            Ok(_v) => Ok(()),
            Err(e) => panic!("error writing to status file: {}", e)
        }
    }

    /// Sets the GPIO port value
    ///
    /// # Arguments
    /// 
    /// * `value` - True to set port value to 1, False to set port value to 0
    pub fn set_value(&self, value: bool) -> Result<(), io::Error> {
        // Determine value to write
        let mut write_v: u8 = 1u8;

        if !value {
            write_v = 0u8;
        }

        let write_v_array: &[u8] = &[write_v];

        // Write to file
        let mut value_f_path = GPIO_PORT_PATH.to_owned();
        value_f_path.push_str("/value");

        let mut value_f = File::create(value_f_path).unwrap();

        value_f.write_all(write_v_array)
    }
}
