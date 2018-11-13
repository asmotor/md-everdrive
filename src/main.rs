#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

extern crate serial;
use serial::prelude::*;

use std::io::Read;

mod arguments;
mod config;
mod everdrive;

fn open_port(port_name: &str) -> Result<Box<serial::SerialPort>, serial::Error> {
    let mut serial_port = serial::open(&port_name)?;
    serial_port.set_timeout(std::time::Duration::from_millis(1000))?;
    everdrive::detect(&mut serial_port)?;
    Ok(Box::new(serial_port))
}

fn load_file(port: &mut SerialPort, filename: &str) -> Result<(), serial::Error> {
    let mut file = std::fs::File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let new_len = (data.len() + 65535) & !65535;
    data.resize(new_len, 0u8);
    
    everdrive::load_data(port, &data)?;
    Ok(())
}

fn run(port_name: &str, options: arguments::RunOptions) -> Result<(), serial::Error> {
    let mut port = open_port(port_name)?;
    load_file(&mut *port, &options.filename)?;
    everdrive::start_image(&mut *port, options.image_type)?;
    Ok(())
}

fn terminal(port: &str) {
}

fn inner_main() -> Result<(), serial::Error> {
    let cfg = config::Config::read();
    if let Some(arguments) = arguments::Arguments::new(cfg) {
        if let Some(port) = arguments.port {
            match arguments.command {
                arguments::Command::Run { options } => {
                    run(&port, options)?;
                }
                arguments::Command::Terminal => {
                    terminal(&port);
                }
            };
        } else {
            return Err(everdrive::error("Port not specified. Either use --port argument or define in configuration."));
        }
    }
    return Ok(());
}

fn main() {
    let exit_code =
        match inner_main() {
            Ok(_) => { 0 }
            Err(msg) => {
                println!("{}", msg);
                1
            }
        };

    std::process::exit(exit_code);
}
