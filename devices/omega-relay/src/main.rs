extern crate byteorder;

pub mod gpio;

fn main() {
    let port1 = gpio::GPIOPort{number: 1u8};

    let status = port1.get_status();
    println!("{:?}", status);

    port1.set_status(gpio::GPIOPortStatus::Exported).unwrap();

    let status = port1.get_status();
    println!("{:?}", status);
}
