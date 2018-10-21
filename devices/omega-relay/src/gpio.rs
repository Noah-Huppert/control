use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::fs::File;

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

/// Indicates if a GPIOPort is setup to output or receive signals
#[derive(Debug)]
pub enum GPIOPortDirection {
    /// Indicates that the GPIO port is setup to output signals
    Out,

    /// Indicates tat the GPIO port is setup to receive signals
    In
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
    pub fn set_status(&self, status: GPIOPortStatus) -> Result<(), String> {
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

        // Write to file
        let mut status_f = try!(File::create(status_f_path)
            .map_err(|e| format!("error opening status file: {}", e)));

        let write_v = format!("{}\n", self.number);
     
        match status_f.write_all(write_v.as_bytes()) {
            Ok(_v) => Ok(()),
            Err(e) => Err(format!("error writing to status file: {}", e))
        }
    }

    /// Returns the value of the GPIO port
    pub fn get_value(&self) -> Result<bool, String> {
        // Read file
        let mut value_f_path = self.get_path();
        value_f_path.push_str("/value");

        let mut value_f = try!(File::open(value_f_path)
            .map_err(|e| format!("error opening value file: {}", e)));

        let mut read_v_array = [0u8; 1];

        try!(value_f.read(&mut read_v_array[..])
            .map_err(|e| format!("error reading value file: {}", e)));

        Ok(read_v_array[0] == 1u8)
    }

    /// Sets the GPIO port value
    ///
    /// # Arguments
    /// 
    /// * `value` - True to set port value to 1, False to set port value to 0
    pub fn set_value(&self, value: bool) -> Result<(), String> {
        // Determine value to write
        let mut write_v: u8 = 1u8;

        if !value {
            write_v = 0u8;
        }

        let write_v_array: &[u8] = &[write_v];

        // Write to file
        let mut value_f_path = self.get_path();
        value_f_path.push_str("/value");

        let mut value_f = try!(File::create(value_f_path)
            .map_err(|e| format!("error opening value file: {}", e)));

        try!(value_f.write_all(write_v_array)
             .map_err(|e| format!("error writing data to value file: {}", e)));

        Ok(())
    }

    /// Returns the direction of the GPIO port
    pub fn get_direction(&self) -> Result<GPIOPortDirection, String> {
        // Read file
        let mut direction_f_path = self.get_path();
        direction_f_path.push_str("/direction");

        let mut direction_f = try!(File::open(direction_f_path)
            .map_err(|e| format!("error opening direction file: {}", e)));

        let mut direction_v = String::new();

        try!(direction_f.read_to_string(&mut direction_v)
             .map_err(|e| format!("error reading direction file: {}", e)));

        if direction_v == "in" {
            Ok(GPIOPortDirection::In)
        } else {
            Ok(GPIOPortDirection::Out)
        }
    }

    /// Sets the GPIO port direction
    ///
    /// # Arguments
    ///
    /// * `value` - Direction to set the port to
    pub fn set_direction(&self, direction: GPIOPortDirection) -> Result<(), String> {
        // Determine value to write
        let direction_v = match direction {
            GPIOPortDirection::In => "in",
            GPIOPortDirection::Out => "out"
        };

        // Write to file
        let mut direction_f_path = self.get_path();
        direction_f_path.push_str("/direction");

        let mut direction_f = try!(File::create(direction_f_path)
            .map_err(|e| format!("error opening direction file: {}", e)));

        try!(direction_f.write_all(direction_v.as_bytes())
             .map_err(|e| format!("error writing to direction file: {}", e)));

        Ok(())
    }
}
