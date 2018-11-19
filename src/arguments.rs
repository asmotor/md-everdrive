extern crate toml;
extern crate dirs;

use config;

#[derive(Debug)]
pub enum ImageType {
    MasterSystem,
    OSUnknown,
    MegaDrive,
    MegaCD,
    JvcXEye,
    SSF2
}

#[derive(Debug)]
pub struct RunOptions {
    pub filename: String,
    pub image_type: ImageType,
    pub terminal: bool
}

#[derive(Debug)]
pub struct OSOptions {
    pub filename: String
}

#[derive(Debug)]
pub struct FPGAOptions {
    pub filename: String
}

#[derive(Debug)]
pub enum Command {
    Run { options: RunOptions },
    OS { options: OSOptions },
    FPGA { options: FPGAOptions },
    Terminal
}

#[derive(Debug)]
pub struct Arguments {
    pub port: Option<String>,
    pub debug: bool,
    pub command: Command
}

impl Arguments {
    fn app() -> clap::App<'static,'static> {
        return clap_app!(mdeverdrive =>
            (@arg PORT: -p --port +takes_value "Serial port to use")
            (@arg DEBUG: --debug "Prints diagnostic messages")
            (@subcommand run =>
                (about: "Uploads and runs binary image")
                (@arg TERMINAL: --terminal "Enters terminal after starting image")
                (@arg SMS: --("master-system") "Selects Master System mode")
                (@arg UNKNOWN: --("unknown") "Selects unknown mode (???)")
                (@arg MEGADRIVE: --("mega-drive") conflicts_with[SMS] "Selects Mega Drive mode (default)")
                (@arg MEGACD: --("mega-cd") conflicts_with[SMS MEGADRIVE] "Selects Mega CD mode")
                (@arg JVCXEYE: --("jvc-xeye") conflicts_with[SMS MEGADRIVE MEGACD] "Selects JVC X'EYE mode")
                (@arg SSF: --ssf conflicts_with[MEGADRIVE SMS MEGACD JVCXEYE] "Selects the extended SSF mapper mode")
                (@arg FILENAME: +required "The binary image to run")
            )
            (@subcommand fpga =>
                (about: "Uploads alternative FPGA bitstream")
                (@arg FILENAME: +required "The bitstream to run")
            )
            (@subcommand os =>
                (about: "Uploads and reboots into alternative OS")
                (@arg FILENAME: +required "The binary image to run")
            )
            (@subcommand terminal =>
                (about: "Enters terminal mode")
            ))
            .author(crate_authors!("\n"))
            .version(crate_version!())
            .about(crate_description!());
    }

    fn new_run_options(matches: &clap::ArgMatches) -> RunOptions {
        let image_type =
            if matches.is_present("SMS") { ImageType::MasterSystem }
            else if matches.is_present("UNKNOWN") { ImageType::OSUnknown }
            else if matches.is_present("MEGACD") { ImageType::MegaCD }
            else if matches.is_present("JVCXEYE") { ImageType::JvcXEye }
            else if matches.is_present("SSF") { ImageType::SSF2 }
            else { ImageType::MegaDrive };

        RunOptions {
            filename: matches.value_of("FILENAME").unwrap().to_string(),
            image_type: image_type,
            terminal: matches.is_present("TERMINAL")
        }
    }

    fn new_os_options(matches: &clap::ArgMatches) -> OSOptions {
        OSOptions {
            filename: matches.value_of("FILENAME").unwrap().to_string()
        }
    }

    fn new_fpga_options(matches: &clap::ArgMatches) -> FPGAOptions {
        FPGAOptions {
            filename: matches.value_of("FILENAME").unwrap().to_string()
        }
    }

    pub fn new(config: config::Config) -> Option<Arguments> {
        let matches = Arguments::app().get_matches();

        let command = 
            if let Some (matches) = matches.subcommand_matches("run") {
                Some (Command::Run { options: Arguments::new_run_options(matches) })
            } else if let Some (matches) = matches.subcommand_matches("os") {
                Some (Command::OS { options: Arguments::new_os_options(matches) })
            } else if let Some (matches) = matches.subcommand_matches("fpga") {
                Some (Command::FPGA { options: Arguments::new_fpga_options(matches) })
            } else if let Some (_) = matches.subcommand_matches("terminal") {
                Some (Command::Terminal)
            } else {
                let _ = Arguments::app().print_help();
                println!();
                None
            };
        
        let port = matches.value_of("PORT").map(|s| s.to_string()).or(config.port);
        let debug = matches.is_present("DEBUG");

        return command.map (|cmd| Arguments { port: port, debug: debug, command: cmd });
    }
}

