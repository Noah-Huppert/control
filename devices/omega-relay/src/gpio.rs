use std::fs;

const GPIO_PORT_PATH: &'static str = "/sys/class/gpio";

pub struct GPIOPort {
    pub number: i8,
}

#[derive(Debug)]
pub enum GPIOPortStatus {
    Exported,
    Unexported
}

impl GPIOPort {
    fn get_path(&self) -> String {
        return format!("{}/gpio{}", GPIO_PORT_PATH, self.number);
    }

    pub fn get_status<'a>(&self) -> GPIOPortStatus {
        let paths = fs::read_dir(GPIO_PORT_PATH).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            let path_str = path.to_str().unwrap();

            if path_str == self.get_path() {
                return GPIOPortStatus::Exported;
            }
        }

        return GPIOPortStatus::Unexported;
    }
}
