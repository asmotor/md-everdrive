#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

extern crate serial;
use serial::prelude::*;

mod arguments;
mod config;
mod everdrive;

fn open_port(port_name: &str) -> Result<Box<serial::SerialPort>, serial::Error> {
    let mut serial_port = serial::open(&port_name)?;
    serial_port.set_timeout(std::time::Duration::from_millis(1000))?;

    Ok(Box::new(serial_port))
}

fn run(port: &str, options: &arguments::RunOptions) {
}

fn terminal(port: &str) {
}

fn inner_main() -> Result<(), String> {
    let cfg = config::Config::read();
    if let Some(arguments) = arguments::Arguments::new(cfg) {
        if let Some(port) = arguments.port {
            match arguments.command {
                arguments::Command::Run { options } => {
                    run(&port, &options);
                }
                arguments::Command::Terminal => {
                    terminal(&port);
                }
            };
        } else {
            return Err("Port not specified. Either use --port argument or define in configuration.".to_string());
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
