extern crate clap;
extern crate sdl2;
use artnet_protocol::*;
use clap::{App, Arg, SubCommand};
use env_logger;
use gif::SetParameter;
use glola::prelude::*;
use hexdump;
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
use std::net::ToSocketAddrs;
use std::net::UdpSocket;
use std::path::Path;
use std::time::Duration;

pub fn main() -> std::io::Result<()> {
    socket.set_broadcast(true).unwrap();
    let buff = ArtCommand::Poll(Poll::default()).into_buffer().unwrap();
    socket.send_to(&buff, &broadcast_addr).unwrap();
    println!("begin loop");
    loop {
        let mut buffer = [0u8; 1024];
        let (length, addr) = socket.recv_from(&mut buffer).unwrap();
        hexdump::hexdump(&buffer[0..length]);
        dbg!(length);
        let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();
        println!("Received {:?}", command);
        match command {
            ArtCommand::Poll(poll) => {
                // This will most likely be our own poll request, as this is broadcast to all devices on the network
            }
            ArtCommand::PollReply(reply) => {
                for x in (0..100).cycle() {
                    // This is an ArtNet node on the network. We can send commands to it like this:
                    for i in 0..=3 {
                        let mut command = ArtCommand::Output(Output {
                            length: 512,        // must match your data.len()
                            data: vec![x; 512], // The data we're sending to the node
                            physical: i,
                            subnet: i as u16,
                            ..Output::default()
                        });
                        let bytes = command.into_buffer().unwrap();
                        socket.send_to(&bytes, &addr).unwrap();
                    }
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
            _ => {}
        }
    }
    Ok(())
}
