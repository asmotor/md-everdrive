#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

mod arguments;
mod config;

fn run (options: &arguments::RunOptions) {
}

fn terminal () {
}

fn main () {
    let cfg = config::Config::read();
    if let Some(arguments) = arguments::Arguments::new(cfg) {
        match arguments.command {
            arguments::Command::Run { options } => {
                run(&options);
            }
            arguments::Command::Terminal => {
                terminal();
            }
        }
    }
}
