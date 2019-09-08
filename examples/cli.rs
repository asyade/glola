use glola::options::Mapping;
use std::io;
extern crate serde;
extern crate serde_json;

fn usage() {
    println!("Usage: cli [command] --config=[CONFIG FILE]");
    println!("  Command list:");
    println!("    dump-addr - print the led mapping from current configuration");
}

fn dump_addr() {}

fn main() -> io::Result<()> {
    let command: Option<String> = None;
    let config: Option<String> = None;
    for arg in std::env::args() {
        if arg == "--help" {
            return Ok(usage());
        }
    }
    if let (Some(command), Some(config)) = (command, config) {
        let cfg: Mapping = serde_json::from_reader(
            std::fs::OpenOptions::new()
                .read(true)
                .open(config)
                .expect("Can't open config file"),
        )
        .expect("Wrong config file");
        Ok(())
    } else {
        return Ok(usage());
    }
}
