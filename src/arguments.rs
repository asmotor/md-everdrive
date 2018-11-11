extern crate toml;
extern crate dirs;

use config;

#[derive(Debug)]
pub enum ImageType {
    Genesis,
    MasterSystem
}

#[derive(Debug)]
pub struct RunOptions {
    pub filename: String,
    pub image_type: ImageType
}

#[derive(Debug)]
pub enum Command {
    Run { options: RunOptions },
    Terminal
}

#[derive(Debug)]
pub struct Arguments {
    pub port: Option<String>,
    pub command: Command
}

impl Arguments {
    fn app() -> clap::App<'static,'static> {
        return clap_app!(mdeverdrive =>
            (version: "0.1.0")
            (author: "Carsten Elton Sorensen <csoren@gmail.com>")
            (about: "Mega Everdrive X3/5/7 interface")
            (@arg PORT: -p --port +takes_value "Serial port to use")
            (@subcommand run =>
                (about: "Uploads and runs binary image")
                (@arg SMS: --sms "Selects Master System mode")
                (@arg FILENAME: +required "The binary image to run")
            )
            (@subcommand terminal =>
                (about: "Enters terminal mode")
            )
        );
    }

    fn new_run_options(matches: &clap::ArgMatches) -> RunOptions {
        let image_type =
            if matches.is_present("SMS") { ImageType::MasterSystem }
            else { ImageType::Genesis };

        RunOptions {
            filename: matches.value_of("FILENAME").unwrap().to_string(),
            image_type: image_type
        }
    }

    pub fn new(config: config::Config) -> Option<Arguments> {
        let matches = Arguments::app().get_matches();

        let command = 
            if let Some (matches) = matches.subcommand_matches("run") {
                Some (Command::Run { options: Arguments::new_run_options(matches) })
            } else if let Some (_) = matches.subcommand_matches("terminal") {
                Some (Command::Terminal)
            } else {
                let _ = Arguments::app().print_help();
                println!();
                None
            };
        
        let port = matches.value_of("PORT").map(|s| s.to_string()).or(config.port);

        return command.map (|cmd| Arguments { port: port, command: cmd });
    }
}

