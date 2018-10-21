extern crate byteorder;

pub mod gpio;

fn main() {
    let port1 = gpio::GPIOPort{number: 1u8};

    // Export port
    println!("===== Export GPIO 1");

    let status = port1.get_status();
    println!("Before: {:?}", status);

    port1.set_status(gpio::GPIOPortStatus::Exported).unwrap();

    let status = port1.get_status();
    println!("After: {:?}", status);

    // Set direction
    println!("===== Set direction");

    let direction = port1.get_direction().unwrap();
    println!("Before: {:?}", direction);

    port1.set_direction(gpio::GPIOPortDirection::Out).unwrap();

    let direction = port1.get_direction().unwrap();
    println!("After: {:?}", direction);

    // Set value
    println!("===== Set GPIO value to 1");
    
    let value = port1.get_value().unwrap();
    println!("Before: {:?}", value);

    port1.set_value(true).unwrap();

    let value = port1.get_value().unwrap();
    println!("After: {:?}", value);

    // Unexport port
    println!("===== Unexport GPIO 1");

    let status = port1.get_status();
    println!("Before: {:?}", status);

    port1.set_status(gpio::GPIOPortStatus::Unexported).unwrap();

    let status = port1.get_status();
    println!("After: {:?}", status);

}
