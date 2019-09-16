extern crate gif;
extern crate serde;
extern crate serde_json;
use gif::SetParameter;
use glola::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;

fn usage() {
    println!("Usage: cli [command]");
    println!("  Command list:");
    println!("    dump-addr - print the led mapping from current configuration");
    println!("    default-cfg - print a default configuration, can be used with `jq` to generate config from scripts");
    println!("    gifloop [GIF FILE] - parse the given GIF file and print it to OLA");
}

fn gif_loop(opt: MappingOpt) {
    let buff_size = opt.height * opt.width;
    let mut buffer: Vec<RGBW> = (0..buff_size).map(|_| RGBW::from(0)).collect();
    let mut decoder = gif::Decoder::new(
        File::open(std::env::args().nth(2).expect("No gif file provided"))
            .expect("Can't open gif file"),
    );
    // Configure the decoder such that it will expand the image to RGBA.
    decoder.set(gif::ColorOutput::RGBA);
    // Read the file header
    let mut decoder = decoder.read_info().unwrap();
    // let environment = Environment::query().unwrap();
    let n = opt.width * opt.height;
    let mut cli = Client::new(opt, |buffer| {
        println!("{}", buffer);
    })
    .unwrap();
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        for x in 0..frame.width {
            for y in 0..frame.height {
                let idx: usize = x as usize * y as usize;
                // pbuff.set(x as u32, y as u32, &color);
                if idx >= buffer.len() {
                    continue;
                }
                buffer[idx] = RGBW::from(frame.buffer[idx] as u32);
            }
        }
        let packet = ScreenBuffer::from(buffer.as_ref());
        cli.apply_buffer(&packet);
    }
}

fn dump_addr(opt: MappingOpt) {
    let addr = AddrMap::from_mapping(opt.into());
    println!("{}", addr);
}

fn default_cfg() {
    let map = MappingOpt {
        width: 10,
        height: 10,
        dmx_size: 10,
        univer_height: 10,
        color_mode: ColorMode::RGBA,
        displacement: Displacement::Snake,
        direction: Direction::Horizontal,
        orientation: Orientation::TopLeft,
    };
    println!("{}", serde_json::to_string_pretty(&map).unwrap());
}

fn main() -> io::Result<()> {
    if let Some(command) = std::env::args().nth(1) {
        if command == "default-cfg" {
            return Ok(default_cfg());
        }
        let mut config = std::env::var("CONFIG").expect("Please set the CONFIG environement variable, for an example config run `cli default-cfg`");
        let cfg: MappingOpt = serde_json::from_reader(
            std::fs::OpenOptions::new()
                .read(true)
                .open(config)
                .expect("Can't open config file"),
        )
        .expect("Wrong config file");
        match command.as_ref() {
            "dump-addr" => dump_addr(cfg),
            "gif-loop" => gif_loop(cfg),
            _ => usage(),
        }
        Ok(())
    } else {
        return Ok(usage());
    }
}
