pub mod gpio;

fn main() {
    let port1 = gpio::GPIOPort{number: 1i8};
    let status = port1.get_status();
    println!("{:?}", status);
}
