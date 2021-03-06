#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

extern crate serialport;
use serialport::*;

use std::io::Read;
use std::io::Write;

mod arguments;
mod config;
mod everdrive;

fn open_port(port_name: &str) -> Result<Box<dyn serialport::SerialPort>> {
    serialport::new(port_name, 57600).timeout(std::time::Duration::from_millis(1000)).open()
}

fn load_padded_data(filename: &str) -> Result<Vec<u8>> {
    let mut file = std::fs::File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let new_len = (data.len() + 65535) & !65535;
    data.resize(new_len, 0u8);

    Ok(data)
}

fn load_bitstream(filename: &str) -> Result<Vec<u8>> {
    let mut file = std::fs::File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    data.resize(0x18000, 0xFFu8);

    Ok(data)
}

fn load_file(port: &mut dyn SerialPort, filename: &str) -> Result<()> {
    let data = load_padded_data(filename)?;
    everdrive::load_data(port, &data)?;
    Ok(())
}

fn load_os(port: &mut dyn SerialPort, filename: &str) -> Result<()> {
    let data = load_padded_data(filename)?;
    everdrive::load_os(port, &data)?;
    Ok(())
}

fn load_fpga(port: &mut dyn SerialPort, filename: &str) -> Result<()> {
    let data = load_bitstream(filename)?;
    everdrive::load_fpga(port, &data)?;
    Ok(())
}

fn terminal_of_port(port: &mut dyn SerialPort) -> Result<()> {
    port.set_timeout(std::time::Duration::from_secs(60 * 60 * 24))?;

    loop {
        let d = everdrive::read_byte(port)?;
        print!("{}", d as char);
        std::io::stdout().flush()?;
    }
}

fn terminal_of_name(port_name: &str) -> Result<()> {
    let mut port = open_port(port_name)?;
    terminal_of_port(&mut *port)?;
    Ok(())
}

fn run(port_name: &str, options: arguments::RunOptions, debug: bool) -> Result<()> {
    let mut port = open_port(port_name)?;
    everdrive::detect(&mut *port, debug)?;
    load_file(&mut *port, &options.filename)?;
    everdrive::start_image(&mut *port, options.image_type, debug)?;
    if options.terminal {
        terminal_of_port(&mut *port)?;
    }
    Ok(())
}

fn os(port_name: &str, options: arguments::OSOptions, debug: bool) -> Result<()> {
    let mut port = open_port(port_name)?;
    everdrive::detect(&mut *port, debug)?;
    load_os(&mut *port, &options.filename)?;

    Ok(())
}

fn fpga(port_name: &str, options: arguments::FPGAOptions, debug: bool) -> Result<()> {
    let mut port = open_port(port_name)?;
    everdrive::detect(&mut *port, debug)?;
    load_fpga(&mut *port, &options.filename)?;

    Ok(())
}

fn inner_main() -> Result<()> {
    let cfg = config::Config::read();
    if let Some(arguments) = arguments::Arguments::new(cfg) {
        if let Some(port) = arguments.port {
            match arguments.command {
                arguments::Command::Run { options } => {
                    run(&port, options, arguments.debug)?;
                }
                arguments::Command::OS { options } => {
                    os(&port, options, arguments.debug)?;
                }
                arguments::Command::FPGA { options } => {
                    fpga(&port, options, arguments.debug)?;
                }
                arguments::Command::Terminal => {
                    terminal_of_name(&port)?;
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
