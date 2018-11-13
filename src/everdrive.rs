extern crate serial;
use serial::prelude::*;

use std::io;
use arguments;

pub fn error(description: &str) -> serial::Error {
    serial::Error::from(io::Error::new(std::io::ErrorKind::Other, description))
}

fn read_byte(port: &mut SerialPort) -> serial::Result<u8> {
    let mut d: [u8; 1] = [0; 1];
    port.read_exact(&mut d)?;
    Ok(d[0])
}

fn write_byte(port: &mut SerialPort, data: u8) -> serial::Result<()> {
    let d: [u8; 1] = [data; 1];
    port.write(&d)?;
    Ok(())
}

fn expect(port: &mut SerialPort, data: u8) -> Result<(), serial::Error> {
    if read_byte(port)? == data {
        Ok(())
    } else {
        Err(error("Unexpected response"))
    }
}

pub fn detect(port: &mut SerialPort) -> Result<(), serial::Error> {
    port.write(b"    *T")?;
    expect(port, b'k')
}

pub fn load_data(port: &mut SerialPort, data: &[u8]) -> Result<(), serial::Error> {
    if data.len() > 0xF00000 {
        Err(error("File size exceeded (maximum 15 MiB)"))
    } else {
        port.write(b"*g")?;
        write_byte(port, (data.len() / 65536) as u8)?;
        expect(port, b'k')?;

        port.write(data)?;
        expect(port, b'd')?;

        Ok(())
    }
}

pub fn start_image(port: &mut SerialPort, image_type: arguments::ImageType) -> Result<(), serial::Error> {
    let cmd =
        match image_type {
            arguments::ImageType::MegaDrive => { b"*rm" }
            arguments::ImageType::MasterSystem => { b"*rs" }
            arguments::ImageType::MegaCD => { b"*rc" }
            arguments::ImageType::JvcXEye => { b"*rM" }
            arguments::ImageType::SSF2 => { b"*rS" }
        };

    port.write(cmd)?;
    expect(port, b'k')?;

    Ok(())
}