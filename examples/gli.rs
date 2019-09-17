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
use sdl2::render::TextureCreator;
use sdl2::render::*;
use sdl2::video::*;
use sdl2::EventPump;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::time::Duration;
const FPS: f64 = 60.0;
const FRAME_DELLAY: f64 = 1000.0 / FPS;
const BW: usize = 1;

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
    sdl_context: sdl2::Sdl,
    opt: MappingOptExt,
    mul: usize,
    canvas: WindowCanvas,
    event_pump: EventPump,
    texture_creator: TextureCreator<sdl2::video::WindowContext>,
}

impl DebugRenderer {
    fn new(mul: usize, opt: MappingOptExt) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let w = ((opt.width * mul) + ((1 + opt.univer_per_column) * BW)) as u32;
        let h = ((opt.height * mul) + ((1 + opt.univer_per_row) * BW)) as u32;
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
        let texture_creator = canvas.texture_creator();
        Self {
            texture_creator,
            event_pump,
            sdl_context,
            canvas,
            mul,
            opt,
        }
    }

    fn poll_event(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                _ => {}
            }
        }
    }

    fn dump(&mut self, packet: &[glola::dmx::ArtDmx]) {
        for (i, u) in packet.iter().enumerate() {
            let x_univer = i % self.opt.univer_per_column;
            let y_univer = i / self.opt.univer_per_column;
            let mut x_offset_in = x_univer * self.opt.univer_width * self.mul;
            let mut y_offset_in = y_univer * self.opt.univer_height * self.mul;
            x_offset_in += BW * (1 + x_univer);
            y_offset_in += BW * (1 + y_univer);
            let mut texture = self
                .texture_creator
                .create_texture_streaming(
                    PixelFormatEnum::RGBA32,
                    self.opt.univer_width as u32,
                    self.opt.univer_height as u32,
                )
                .map_err(|e| e.to_string())
                .expect("Failed to create texture");
            // Create a red-green gradient
            texture
                .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                    for y in 0..self.opt.univer_height {
                        for x in 0..self.opt.univer_width {
                            let offset = y * pitch + x * 4;
                            buffer[offset] = u.data[offset];
                            buffer[offset + 1] = u.data[offset + 1];
                            buffer[offset + 2] = u.data[offset + 2];
                            buffer[offset + 3] = u.data[offset + 3];
                        }
                    }
                })
                .expect("Filed to stream texture");
            self.canvas
                .copy(
                    &texture,
                    None,
                    Some(Rect::new(
                        x_offset_in as i32,
                        y_offset_in as i32,
                        (self.opt.univer_height * self.mul) as u32,
                        (self.opt.univer_width * self.mul) as u32,
                    )),
                )
                .expect("Failed to apply texture");
        }
        self.canvas.present();
    }
}

fn gif_loop(gif: &str, opt: MappingOpt, hexd: bool, mul: usize, window: bool) {
    let mut screen = glola::init_arnet_screen(opt.clone());
    let opt: MappingOptExt = opt.into();
    let mut decoder = gif::Decoder::new(File::open(gif).unwrap());
    decoder.set(gif::ColorOutput::RGBA);
    let mut decoder = decoder.read_info().unwrap();
    let mut sized_frames: Vec<Vec<u8>> = vec![];
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        if opt.width * opt.height * opt.pixel_size >= frame.buffer.len() {
            let mut frm = frame.buffer.to_vec();
            let miss = opt.width * opt.height * opt.pixel_size - frame.buffer.len();
            for _ in 0..miss {
                frm.push(0x0);
            }
            sized_frames.push(frm);
        } else {
            sized_frames.push(frame.buffer[0..opt.width * opt.height * opt.pixel_size].to_vec());
        }
    }
    let mut cycle = sized_frames.iter().cycle();
    let mut dbg = if window {
        Some(DebugRenderer::new(mul, opt.clone().into()))
    } else {
        None
    };
    'running: while let Some(frame) = cycle.next() {
        let instant = std::time::Instant::now();
        let _ = dbg.as_mut().map(|e| e.poll_event());
        let (fps, packet) = screen.apply(frame);
        for univer in packet {}
        if hexd {
            for u in packet.iter() {
                println!("{}", u)
            }
        }
        let _ = dbg.as_mut().map(|e| e.dump(packet));
        regulate_fps!(instant.elapsed().as_millis() as f64, FRAME_DELLAY);
    }
}
use sdl2::pixels::PixelFormatEnum;
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
                    Arg::with_name("window")
                        .short("w")
                        .help("Debug matrix on a sdl2 window.")
                )
                .arg(
                    Arg::with_name("multiplier")
                        .requires("window")
                        .short("m")
                        .takes_value(true)
                        .help("Size of a pixel in the visual interface")
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
        let window = cmd.is_present("window");
        let multiplier: usize = if cmd.is_present("multiplier") {
            String::from(cmd.value_of("multiplier").unwrap())
        } else {
            String::from("5")
        }
        .parse::<usize>()
        .expect("Multiplier must be a positive integer");
        gif_loop(gif, config, hexdump, multiplier, window);
    } else if let Some(cmd) = matches.subcommand_matches("dump") {
        dump(config!(cmd.value_of("config").unwrap()));
    } else {
        error!("No subcommand provided !");
    }
}
