# What is this?
This is a frontend for loading images via USB into a Mega Everdrive X7 cartridge, where they will be instantly run on the attached Sega Mega Drive. This provides a fairly pleasant build/deploy cycle for Mega Drive homebrew.

This tool is is rewrite of the official usb-tool.exe, which only works well on Windows. This tool has focus on macOS and Linux, but may work on Windows as well.

# How to use
    USAGE:
        everdrivemd [FLAGS] [OPTIONS] [SUBCOMMAND]

    FLAGS:
            --debug      Prints diagnostic messages
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -p, --port <PORT>    Serial port to use

    SUBCOMMANDS:
        fpga        Uploads alternative FPGA bitstream
        help        Prints this message or the help of the given subcommand(s)
        os          Uploads and reboots into alternative OS
        run         Uploads and runs binary image
        terminal    Enters terminal mode

## The "run" command
Uploads and runs binary image

    USAGE:
        everdrivemd run [FLAGS] <FILENAME>

    FLAGS:
            --mega-drive-10m    Selects Mega Drive 10MiB mode
            --mega-cd           Selects Mega CD BIOS mode
            --mega-drive        Selects Mega Drive mode (default)
            --osapp             Selects OS app
            --master-system     Selects Master System mode
            --ssf               Selects the extended SSF mapper mode
            --terminal          Enters terminal after starting image
            --32x               Selects 32X mode
        -h, --help              Prints help information
        -V, --version           Prints version information

    ARGS:
        <FILENAME>    The binary image to run

# Configuration
Specifying the --port argument quickly gets tedious. To avoid having to do that, create a ```~/.everdrivemd``` file with the contents 

    port = "/dev/tty.usbserial-XXXXXXXX"

Where the path is the port your X7 uses.

# Installation
Currently you will need to have Rust (and Cargo) installed. Then run

    cargo install --force --path .

## Drivers
You may need to install a virtual COM port driver.

### macOS
https://www.ftdichip.com/Drivers/VCP.htm
