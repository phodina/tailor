extern crate svd_parser;
extern crate tera;

use std::fs::File;
use std::io::Read;

use tera::Context;

fn main() {

    let xml = &mut String::new();
    File::open("STM32F30x.svd").unwrap().read_to_string(xml);

    let device : svd_parser::Device = svd_parser::parse(xml);

    for peripheral in device.peripherals {
    	println!("{:?}", peripheral.name);
    	
    	let mut context = Context::new();
    	context.add("peripheral", &peripheral);

    	generate_file("peripheralNAME.rs","peripheral.rs", &context);
    }
}

fn generate_file(dst: &str, src: &str, context : &Context) {
    
}
