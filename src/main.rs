#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate generator;

use std::path::Path;
use std::env;
use std::fs::File;

use generator::{Context,Generator};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Device{
    name: String,
    version: String,
    description: String,
    addressUnitBits: String,
    width: String,
    size: String,
    resetValue: String,
    resetMask: String,
    peripherals: Peripherals,
}

#[derive(Serialize, Deserialize, Debug)]
struct Peripherals{
    #[serde(rename="peripheral")]peripherals: Vec<Peripheral>,
}

#[allow(non_snake_case)] 
#[derive(Serialize, Deserialize, Debug)]
struct Fields{
    #[serde(rename="field")]fields: Vec<Field>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Field {
    name: String,
    description: String,
    bitOffset: String,
    bitWidth: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Registers{
    #[serde(rename="register")]registers: Vec<Register>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Register {
    name: String,
    description: String,
    addressOffset: String,
    size: String,
    access: Option<String>,
    resetValue: String,
    fields: Option<Fields>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddressBlock {
    offset: String,
    size: String,
    usage: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Peripheral {
    name: String,
    description: Option<String>,
    groupName: Option<String>,
    baseAddress: String,
    addressBlock: Option<AddressBlock>,
    registers: Option<Registers>,
}

mod memory_map {

    use super::*;

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct MemoryMap {
        name: String,
        description: String,
        addressUnitBits: u8,
        width: u8,
        size: u32,
        resetValue: u32,
        resetMask: u32,
        pub peripherals: Vec<Peripheral>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Peripheral {
        pub name: String,
        registers: Vec<Register>
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Register {
        fields: Vec<Field>
    }
    
    #[derive(Serialize, Deserialize, Debug)]
    struct Field {
        
    }
    
    impl MemoryMap {
        
        pub fn new (device: Device) -> MemoryMap {

            MemoryMap{
                name: device.name,
                description: device.description,
                addressUnitBits: device.addressUnitBits.parse::<u8>().unwrap(),
                width: device.width.parse::<u8>().unwrap(),
                size: device.size.parse::<u32>().unwrap(),
                resetValue: device.resetValue.parse::<u32>().unwrap(),
                resetMask: device.resetMask.parse::<u32>().unwrap(),
                peripherals: MemoryMap::peripherals(device.peripherals),
            }
        }

        fn peripherals(peripherals_dev: Peripherals) -> Vec<Peripheral> {
            let mut peripherals = Vec::with_capacity(peripherals_dev.peripherals.len());

            for peripheral in peripherals_dev.peripherals {
                let p = match peripheral.registers {
                    Some(registers) => Peripheral{name: peripheral.name, registers: MemoryMap::registers(registers)},
                    None => Peripheral{name: peripheral.name, registers: Vec::new()}
                };

                peripherals.push(p);
            }
          
            peripherals
        }

        fn registers(registers_dev: Registers) -> Vec<Register> {
            let mut registers = Vec::with_capacity(registers_dev.registers.len());
            for register in registers_dev.registers {
                let r = match register.fields {
                    Some(fields) => Register{fields: MemoryMap::fields(fields)},
                    None => Register{fields: Vec::new()}
                };

                registers.push(r);
            }
            
            registers
        }

        fn fields(fields_dev: Fields) -> Vec<Field>{

            let mut fields = Vec::with_capacity(fields_dev.fields.len());

            for field in fields_dev.fields{
                let f = Field{};
                fields.push(f);
            }
            
            fields
        }
    }
}

fn main() {
    let path = env::current_dir().unwrap();
    let template_path = path.to_str().unwrap().to_string();

    let file = File::open("STM32F30x.svd").unwrap();

    let svd: Device = serde_xml_rs::deserialize(file).unwrap();
    let mm = memory_map::MemoryMap::new(svd);
    
    let generator = Generator::new(Path::new("project"), Path::new("template")).unwrap();
    
    for peripheral in mm.peripherals {
	    let mut context = Context::new();
	    context.add("peripheral", &peripheral);
	    let mut filename = peripheral.name.clone();
	    filename.push_str(".rs");

	    generator.generate_file(&context,Path::new("peripheral.rs"), Path::new(&filename)).unwrap();
	}
}
