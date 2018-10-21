extern crate byteorder;

pub mod gpio;

fn main() {
    let port1 = gpio::GPIOPort{number: 1u8};

    // Set value
    println!("===== Set GPIO value to 1");
    
    let value = port1.get_value().unwrap();
    println!("Before: {:?}", value);

    port1.set_value(true).unwrap();

    let value = port1.get_value().unwrap();
    println!("After: {:?}", value);
}
