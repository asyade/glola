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
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::render::*;
use sdl2::video::*;
use sdl2::EventPump;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Cursor;
use std::io::{Read, Seek, Write};
use std::path::Path;
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
    matrix: RevAddrMap,
    mul: usize,
    canvas: WindowCanvas,
    event_pump: EventPump,
    texture_creator: TextureCreator<sdl2::video::WindowContext>,
}

impl DebugRenderer {
    fn new(mul: usize, opt: MappingOptExt) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let w = (opt.width * mul) as u32;
        let h = (opt.height * mul) as u32;
        let window = video_subsystem
            .window("GLOLA", w, h)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.clear();
        canvas.present();
        Self {
            matrix: AddrMap::from_mapping(opt.clone()).into(),
            texture_creator: canvas.texture_creator(),
            event_pump: sdl_context.event_pump().unwrap(),
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
        for (univer_idx, u) in packet.iter().enumerate() {
            let x_univer = univer_idx % self.opt.univer_per_column;
            let y_univer = univer_idx / self.opt.univer_per_column;
            let x_offset_in = x_univer * self.opt.univer_width * self.mul;
            let y_offset_in = y_univer * self.opt.univer_height * self.mul;
            let mut texture = self
                .texture_creator
                .create_texture_streaming(
                    PixelFormatEnum::RGBA32,
                    self.opt.univer_width as u32,
                    self.opt.univer_height as u32,
                )
                .map_err(|e| e.to_string())
                .expect("Failed to create texture");
            texture
                .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                    let mut idx = 0;
                    let map = &self.matrix.offset[univer_idx];
                    for idx in 0..buffer.len() / self.opt.pixel_size {
                        let mapped_offset = map[idx] * self.opt.pixel_size;
                        let off = idx * self.opt.pixel_size;
                        for i in 0..4 {
                            buffer[mapped_offset + i] = u.data[off + i];
                        }
                        // buffer[mapped_offset + 3] = std::u8::MAX - u.data[off + 3];
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

struct GifLoader {
    pub frames: Vec<(Duration, Vec<u8>)>,
}

impl GifLoader {
    fn merge_alpha(old: &[u8], new: &mut [u8]) {
        let mut idx = 3;
        while idx < std::cmp::min(old.len(), new.len()) {
            if new[idx] == 0 && old[idx] != 0 {
                dbg!("Merge");
                new[idx] = old[idx];
            }
            idx += 4;
        }
    }

    fn load<T: AsRef<Path>>(path: T, opt: &MappingOptExt) -> io::Result<Self> {
        let mut decoder = gif::Decoder::new(File::open(path)?);
        decoder.set(gif::ColorOutput::RGBA);
        let mut decoder = decoder
            .read_info()
            .map_err(|_| io::Error::last_os_error())?;
        let mut sized_frames = vec![];
        let mut last: Option<Vec<u8>> = None;
        while let Some(mut frame) = decoder
            .read_next_frame()
            .map_err(|_| io::Error::last_os_error())?
        {
            let mut new_frame = vec![0; opt.width * opt.height * opt.pixel_size];
            let mut curr = frame.buffer.to_vec();
            if let Some(last) = last {
                Self::merge_alpha(&last, &mut curr);
            }
            let height_max = std::cmp::min(frame.height as usize, opt.height);
            let mut rd = Cursor::new(&curr);
            let mut wr = Cursor::new(&mut new_frame);
            let mut line: Vec<u8> = vec![0; opt.width * opt.pixel_size];
            if (frame.width as usize) < opt.width {
                for y in 0..height_max {
                    rd.read_exact(&mut line[0..(frame.width as usize * opt.pixel_size)])?;
                    wr.write_all(&line)?;
                }
            } else {
                for y in 0..height_max {
                    rd.read_exact(&mut line[0..opt.width * opt.pixel_size])?;
                    rd.seek(std::io::SeekFrom::Current(
                        (frame.width as i64 - opt.width as i64) * opt.pixel_size as i64,
                    ))?;
                    wr.write_all(&line)?;
                }
            }
            last = Some(curr);
            sized_frames.push((Duration::from_millis((frame.delay as u64) * 10), new_frame));
        }
        Ok(GifLoader {
            frames: sized_frames,
        })
    }
}

fn gif_loop(gif: &str, opt: MappingOpt, hexd: bool, mul: usize, window: bool) {
    let mut screen = glola::init_arnet_screen(opt.clone());
    let opt: MappingOptExt = opt.into();
    let gif = GifLoader::load(gif, &opt).expect("Wrong gif file !");
    let mut dbg = if window {
        Some(DebugRenderer::new(mul, opt.clone()))
    } else {
        None
    };
    let mut cycle = gif.frames.iter().cycle();
    for frame in cycle {
        let instant = std::time::Instant::now();
        let frame_duration = std::time::Instant::now();
        // We're sending 60 fps even if the gif do not have this frame rate to ensure the matrix can handle it
        while frame_duration.elapsed() < frame.0 {
            let instant = std::time::Instant::now();
            let _ = dbg.as_mut().map(|e| e.poll_event());
            let (fps, packet) = screen.apply(&frame.1);
            println!("fps: {}", fps);
            if hexd {
                for u in packet.iter() {
                    println!("{}", u)
                }
            }
            let _ = dbg.as_mut().map(|e| e.dump(packet));
            regulate_fps!(instant.elapsed().as_millis() as f64, FRAME_DELLAY);
        }
    }
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
                .about("Loads a GIF image and sends its frames from an infinite loop.")
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .help("Matrix configuration file.")

                        .long_help("The matrix configuration is done using a json file, to check a configuration you can use the subcommand dump, example config for a 10x10 matrix that use DMX512 : `{\"dmx_size\": 512,\"width\": 30,\"height\": 30,\"univer_height\": 10,\"channel_per_pixel\": 10,\"color_mode\": \"RGBA\",\"displacement\": \"Snake\",\"direction\": \"Horizontal\",\"orientation\": \"TopLeft\"}`")
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
                        .help("Parse and display in a window the outgoing packet content.")
                )
                .arg(
                    Arg::with_name("multiplier")
                        .requires("window")
                        .short("m")
                        .takes_value(true)
                        .help("Pixel size multiplier display on the screen (use low value for large matrix and increase it for small one).")
                )
                .arg(
                    Arg::with_name("hexdump")
                        .short("h")
                        .help("Hexdump every outgoing packet on the standard output.")
                ),
        )
        .subcommand(
            SubCommand::with_name("dump")
                .about("Inspect a configuration file and dump addresses map.")
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .help("Configuration to inspecte")
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
