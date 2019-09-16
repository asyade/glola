extern crate clap;
extern crate sdl2;
use clap::{App, Arg, SubCommand};
use env_logger;
use gif::SetParameter;
use glola::prelude::*;
use log::{debug, error, info, warn};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::time::Duration;

const FPS: f64 = 60.0;
const FRAME_DELLAY: f64 = 1000.0 / FPS;
const BW: usize = 5;
const MUL: usize = 10;

macro_rules! regulate_fps {
    ($last_delay:expr, $wante_dellay:expr) => {
        if $last_delay < $wante_dellay {
            std::thread::sleep(std::time::Duration::from_millis(
                ($wante_dellay - $last_delay) as u64,
            ));
        }
    };
}

struct DebugRenderer {
    ctx: sdl2::Sdl,
}

fn gif_loop(gif: &str, opt: MappingOpt, visual: bool, hexd: bool) {
    let mut screen = glola::init_arnet_screen(opt.clone());
    let opt: MappingOptExt = opt.into();
    let mut decoder = gif::Decoder::new(File::open(gif).unwrap());
    decoder.set(gif::ColorOutput::RGBA);
    let mut decoder = decoder.read_info().unwrap();
    let mut sized_frames: Vec<Vec<u8>> = vec![];
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        sized_frames.push(frame.buffer[0..opt.width * opt.height * opt.pixel_size].to_vec());
    }
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let w = ((opt.width * MUL) + ((1 + opt.univer_per_column) * BW)) as u32;
    let h = ((opt.height * MUL) + ((1 + opt.univer_per_row) * BW)) as u32;
    let window = video_subsystem
        .window("GLOLA", w, h)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cycle = sized_frames.iter().cycle();
    canvas.set_draw_color(Color::RGB(10, 210, 0));
    // A draw a rectangle which almost fills our window with it !
    for x in 0..=opt.univer_per_column {
        let x = (x * opt.univer_width * MUL) + (BW * x);
        canvas
            .fill_rect(Rect::new(x as i32, 0, BW as u32, h))
            .unwrap();
        for y in 0..=opt.univer_per_row {
            let y = (y * opt.univer_height * MUL) + (BW * y);
            canvas
                .fill_rect(Rect::new(0, y as i32, w, BW as u32))
                .unwrap();
        }
    }
    'running: while let Some(frame) = cycle.next() {
        let instant = std::time::Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        let (fps, packet) = screen.apply(frame);
        for univer in packet {}
        canvas.present();
        if hexd {
            for u in packet.iter() {
                let 
            }
        }
        regulate_fps!(instant.elapsed().as_millis() as f64, FRAME_DELLAY);
    }
}

trait DisplaySdl  {
    fn display_sdl<'a>(&'a self) -> (i32, i32, usize, usize, &'a [u8]);
}

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
                )
                .arg(
                    Arg::with_name("visual")
                        .short("v")
                        .help("Visualize Artnet packet using graphical interface")
                )
                .arg(
                    Arg::with_name("hexdump")
                        .short("h")
                        .help("Dump dmx packet to stdout")
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
        let hexdump = cmd.is_present("hexdump");
        gif_loop(gif, config, cmd.is_present("visual"), hexdump);
    } else if let Some(cmd) = matches.subcommand_matches("dump") {
        dump(config!(cmd.value_of("config").unwrap()));
    } else {
        error!("No subcommand provided !");
    }
}
