use std::{self, io};

use rusb::GlobalContext;
use rusb::{Context, Device, DeviceHandle, UsbContext};

fn list_devices() {
    let context = Context::new().unwrap();

    let devices = context.devices().unwrap();

    for device in devices.iter() {
        let desc = device.device_descriptor().unwrap();
        let handle = device.open().unwrap();

        let serial = handle.read_serial_number_string_ascii(&desc).unwrap();
        let product = handle.read_product_string_ascii(&desc).unwrap();

        println!("serial number of {} is {}", product, serial);
    }
}

fn get_info(wanted_serial: &str) -> rusb::Result<DeviceHandle<GlobalContext>> {
    let devices = rusb::devices()?;

    for device in devices.iter() {
        let desc = device.device_descriptor()?;

        let handle = match device.open() {
            Ok(h) => h,
            Err(_) => continue,
        };

        let serial = handle
            .read_serial_number_string_ascii(&desc)
            .unwrap_or_default();

        if serial == wanted_serial {
            return Ok(handle);
        }
    }

    Err(rusb::Error::NoDevice)
}

pub fn set_up() -> DeviceHandle<GlobalContext> {
    loop {
        list_devices();

        let mut serial_num = String::new();
        println!("what is your device serial number?");
        io::stdin().read_line(&mut serial_num).unwrap();

        if (serial_num.len() < 1) {
            continue;
        }

        let handle = match get_info(&serial_num) {
            Ok(h) => return h,
            Err(rusb::Error::NoDevice) => continue,
            Err(_) => panic!("idk"),
        };

        //creat the screen
        print!("\x1B[2J\x1B[1;1H");
    }
}
