extern crate serial;
use serial::prelude::*;

use std::io;

fn read_byte(port: &mut SerialPort) -> serial::Result<u8> {
    let mut d: [u8; 1] = [0; 1];
    port.read_exact(&mut d)?;
    Ok(d[0])
}

fn write_byte(port: &mut SerialPort, data: u8) -> serial::Result<()> {
    let mut d: [u8; 1] = [data; 1];
    port.write(&d)?;
    Ok(())
}

fn error(description: &str) -> serial::Error {
    serial::Error::from(io::Error::new(std::io::ErrorKind::Other, description))
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
        write_byte(port, (data.len() / 512 / 128) as u8);
        expect(port, b'k')?;

        port.write(data);
        expect(port, b'd')?;

        Ok(())
    }
}