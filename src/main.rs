#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate generator;
#[macro_use]
extern crate error_chain;

mod errors;

use errors::*;

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
    use std::u8;

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
        description: String,
        base_address: u32,
        registers: Vec<Register>
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Register {
        name: String,
        description: String,
        address_offset: u32,
        reset_value: u32,
        fields: Vec<Field>
    }
    
    #[derive(Serialize, Deserialize, Debug)]
    struct Field {
        name: String,
        description: String,
        bit_offset: u8,
        bit_width: u8
    }
    
    impl MemoryMap {
        
        pub fn new (device: Device) -> Result<MemoryMap> {

            Ok(MemoryMap{
                name: device.name,
                description: device.description,
                addressUnitBits: device.addressUnitBits.parse::<u8>().chain_err(|| "Unable to parse addressUnitbits")?,
                width: device.width.parse::<u8>().chain_err(|| "Unable to parse width")?,
                size: u32::from_str_radix(&device.size[2..],16).chain_err(|| "Unable to parse size")?,
                resetValue: u32::from_str_radix(&device.resetValue[2..],16).chain_err(|| "Unable to parse resetValue")?,
                resetMask: u32::from_str_radix(&device.resetMask[2..],16).chain_err(|| "Unable to parse resetMask")?,
                peripherals: MemoryMap::peripherals(device.peripherals)?,
            })
        }

        fn peripherals(peripherals_dev: Peripherals) -> Result<Vec<Peripheral>> {
            let mut peripherals = Vec::with_capacity(peripherals_dev.peripherals.len());

            for peripheral in peripherals_dev.peripherals {
                let rs = match peripheral.registers {
                    Some(registers) => MemoryMap::registers(registers)?,
                    None => Vec::new()
                };

                let description = match peripheral.description {
                    Some(des) => des,
                    None => String::new()
                };
                
                let p = Peripheral{name: peripheral.name,
                                   description: description,
                                   base_address: u32::from_str_radix(&peripheral.baseAddress[2..],16).chain_err(|| "Unable to parse baseAddress")?,
                                   registers: rs};

                peripherals.push(p);
            }
          
            Ok(peripherals)
        }

        fn registers(registers_dev: Registers) -> Result<Vec<Register>> {
            let mut registers = Vec::with_capacity(registers_dev.registers.len());
            for register in registers_dev.registers {
                let fs = match register.fields {
                    Some(fields) => MemoryMap::fields(fields)?,
                    None => Vec::new()
                };

                let r = Register{name: register.name,
                                 description: register.description,
                                 address_offset: u32::from_str_radix(&register.addressOffset[2..],16).chain_err(|| "Unable to parse AddressOffset")?,
                                 reset_value: u32::from_str_radix(&register.resetValue[2..],16).chain_err(|| "Unable to parse ResetValue")?,
                                 fields: fs};

                registers.push(r);
            }
            
            Ok(registers)
        }

        fn fields(fields_dev: Fields) -> Result<Vec<Field>>{

            let mut fields = Vec::with_capacity(fields_dev.fields.len());

            for field in fields_dev.fields{
                let f = Field{name: field.name,
                              description: field.description,
                              bit_width: field.bitWidth.parse::<u8>().chain_err(|| "Unable to parse BitWidth")?,
                              bit_offset: field.bitOffset.parse::<u8>().chain_err(|| "Unable to parse BitOffset")?};
                fields.push(f);
            }
            
            Ok(fields)
        }
    }
}

fn main() {
    let path = env::current_dir().unwrap();
    let template_path = path.to_str().unwrap().to_string();

    let file = File::open("STM32F30x.svd").unwrap();

    let svd: Device = serde_xml_rs::deserialize(file).unwrap();
    let mm = memory_map::MemoryMap::new(svd);

    let generator = Generator::new(Path::new("project"), Path::new("templates")).unwrap();
    
    if let Err(ref e) = mm {

        println!("error: {}", e);

        for e in e.iter().skip(1){
            println!("caused by: {}", e);
        }

        ::std::process::exit(1);
    }
    
    
    for peripheral in mm.unwrap().peripherals {
	    let mut context = Context::new();
	    context.add("peripheral", &peripheral);
	    let mut filename = peripheral.name.clone();
	    filename.push_str(".rs");

	    generator.generate_file(&context,Path::new("peripheral.rs"), Path::new(&filename)).unwrap();
	}
}
