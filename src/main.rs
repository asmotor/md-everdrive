#[macro_use]
extern crate clap;

enum Command {
    Run
}

struct Arguments {
    command: Command
}

impl Arguments {
    fn new() -> Arguments {
        let matches = clap_app!(myapp =>
            (version: "0.1.0")
            (author: "Carsten Elton Sorensen <csoren@gmail.com>")
            (about: "Mega Everdrive X3/5/7 interface")
            (@subcommand run =>
                (about: "uploads and runs binary image")
                (@arg sms: -sms --master-system "Selects Master System mode")
            )
        ).get_matches();
    }
}

fn main () {
    println!("Hello, world!");
}
