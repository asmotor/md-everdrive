extern crate serialport;
use serialport::SerialPort;

use std::io;
use arguments;

pub fn error(description: &str) -> serialport::Error {
    serialport::Error::from(io::Error::new(std::io::ErrorKind::Other, description))
}

pub fn read_byte(port: &mut dyn SerialPort) -> serialport::Result<u8> {
    let mut d: [u8; 1] = [0; 1];
    port.read_exact(&mut d)?;
    Ok(d[0])
}

fn write_byte(port: &mut dyn SerialPort, data: u8) -> serialport::Result<()> {
    let d: [u8; 1] = [data; 1];
    port.write(&d)?;
    Ok(())
}

fn expect(port: &mut dyn SerialPort, data: u8) -> Result<(), serialport::Error> {
    let data_read = read_byte(port)?;
    if data_read == data {
        Ok(())
    } else {
        Err(error(&format!("Unexpected response {}", data_read)))
    }
}

fn flush_read_buffer(port: &mut dyn SerialPort, debug: bool) -> Result<(), serialport::Error> {
    if debug { println!("Flushing read buffer"); }

    port.set_timeout(std::time::Duration::from_millis(100))?;
    while read_byte(port).is_ok() {}
    port.set_timeout(std::time::Duration::from_millis(1000))?;
    Ok(())
}

pub fn detect(port: &mut dyn SerialPort, debug: bool) -> Result<(), serialport::Error> {
    flush_read_buffer(port, debug)?;

    if debug { println!("Sending detect"); }

    port.write(b"    *T")?;
    expect(port, b'k')
}

pub fn load_data(port: &mut dyn SerialPort, data: &[u8]) -> Result<(), serialport::Error> {
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

pub fn load_os(port: &mut dyn SerialPort, data: &[u8]) -> Result<(), serialport::Error> {
    if data.len() > 0xF00000 {
        Err(error("File size exceeded (maximum 15 MiB)"))
    } else {
        let blocks = data.len() / 512;
        port.write(b"*o")?;
        write_byte(port, (blocks >> 8) as u8)?;
        write_byte(port, blocks as u8)?;

        port.write(data)?;
        port.write(b"*R")?;

        Ok(())
    }
}

pub fn load_fpga(port: &mut dyn SerialPort, data: &[u8]) -> Result<(), serialport::Error> {
    if data.len() != 0x18000 {
        Err(error("Wrong size for RBF"))
    } else {
        let len = data.len() / 2;
        port.write(b"*f")?;
        write_byte(port, (len >> 8) as u8)?;
        write_byte(port, len as u8)?;

        port.write(data)?;

        Ok(())
    }
}

pub fn start_image(port: &mut dyn SerialPort, image_type: arguments::ImageType, debug: bool) -> Result<(), serialport::Error> {
    let cmd =
        match image_type {
            arguments::ImageType::MegaDrive => { b"*rm" }
            arguments::ImageType::OSApp => { b"*ro" }
            arguments::ImageType::MasterSystem => { b"*rs" }
            arguments::ImageType::MegaCD => { b"*rc" }
            arguments::ImageType::MegaDrive10M => { b"*rM" }
            arguments::ImageType::SSF2 => { b"*rS" }
            arguments::ImageType::X32 => { b"*r3" }
        };

    if debug { println!("Starting image"); }

    port.write(cmd)?;
    expect(port, b'k')?;

    Ok(())
}