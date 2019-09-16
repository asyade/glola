//!
//! Based on https://artisticlicence.com/WebSiteMaster/User%20Guides/art-net.pdf
//!

use super::Encoder;
use crate::dmx::ArtDmx;
use crate::prelude::*;

pub struct ArtnetEncoder {
    opt: MappingOptExt,
    univers: Vec<ArtDmx>,
}

impl std::fmt::Display for ArtDmx {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "ID: {:?}, OP: {:?}, VER: {:?}, LEN: {:?}\n",
            self.id, self.op_code, self.proto_ver, self.lenght
        )?;
        for line in hexdump::hexdump_iter(&self.data[0..self.lenght as usize]) {
            write!(fmt, "{}\n", line)?;
        }
        Ok(())
    }
}

impl ArtDmx {
    fn new(opt: &MappingOptExt) -> Self {
        Self {
            id: [b'A', b'r', b't', b'-', b'N', b'e', b't', b'\0'],
            op_code: 0x5000, // OpOutput
            proto_ver: [5, 57],
            sequence: 0,
            physical: 0,
            sub_uni: 0, //TODO
            net: 0,     //TODO
            lenght: (opt.univer_width * opt.univer_height) as u16,
            data: [0; 512],
        }
    }
}

impl ArtnetEncoder {
    pub fn new(opt: MappingOptExt) -> Self {
        Self {
            univers: vec![ArtDmx::new(&opt); opt.univer_per_column * opt.univer_per_row],
            opt,
        }
    }
}

impl Encoder for ArtnetEncoder {
    fn encode(&mut self, matrix: &AddrMap, buffer: &[u8]) {
        for x in 0..self.opt.width {
            for y in 0..self.opt.height {
                let addr = &matrix.addr[x][y];
                let offset = x + (y * self.opt.width);
                self.univers[addr.univer].data[addr.address..addr.address + self.opt.pixel_size]
                    .copy_from_slice(&buffer[offset..offset + self.opt.pixel_size]);
            }
        }

        for i in self.univers.iter() {
            println!("{}", i);
        }
    }
}
