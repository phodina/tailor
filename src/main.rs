extern crate svd_parser;

use std::fs::File;
use std::io::Read;

fn main() {

    let xml = &mut String::new();
    File::open("STM32F30x.svd").unwrap().read_to_string(xml);

    let device : svd_parser::Device = svd_parser::parse(xml);

    for peripheral in device.peripherals {
    	println!("{:?}", peripheral.name);
    }
    // TODO gen file
}
