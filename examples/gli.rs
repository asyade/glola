extern crate clap;
use clap::{App, Arg, SubCommand};
use env_logger;
use gif::SetParameter;
use glola::prelude::*;
use log::{debug, error, info, warn};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;

fn gif_loop() {}

fn dump(opt: MappingOpt) {
    let opt: MappingOptExt = opt.into();
    let addr = AddrMap::from_mapping(opt.clone());
    println!("{}{:?}", addr, opt);
}

macro_rules! config {
    ($config: expr) => {{
        let mut file = OpenOptions::new()
            .read(true)
            .open($config)
            .expect("Can't access configuration file !");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Can't read configuration file !");
        let res: MappingOpt = serde_json::from_str(&buf).expect("Can't parse configuration file !");
        res
    }};
}

fn main() {
    env_logger::init();
    let matches = App::new("GLOLA CLI")
        .version("0.4.2")
        .author("Asya .C. <asya.corbeau@student.42.fr>")
        .about("A CLI backed with OLA/OLA to control huge led matrix controlled using many arnet univer.")
        .subcommand(
            SubCommand::with_name("gif")
                .about("Send a GIF file frame by frame to the matrix.")
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .help("Configuration file (json).")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("gif")
                        .short("g")
                        .help("GIF file to parse.")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("dump")
                .about("Inspect a configuration file and dump address map.")
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .help("Configuration file (json).")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();
    if let Some(cmd) = matches.subcommand_matches("gif") {
        let config = config!(cmd.value_of("config").unwrap());
        let gif = cmd.value_of("gif").unwrap();
    } else if let Some(cmd) = matches.subcommand_matches("dump") {
        dump(config!(cmd.value_of("config").unwrap()));
    } else {
        error!("No subcommand provided !");
        return;
    }
}
