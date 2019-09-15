//!
//! `OLA` client wrapper that can map `ScreenBuffer` into DXM packet/arnet univers
//!
use crate::prelude::{AddrMap, GError, Mapping, ScreenBuffer, RGBW};
use std::fmt;

pub struct DXMPacket(Vec<u8>);

impl DXMPacket {
    #[inline(always)]
    pub fn set_pixel(&mut self, idx: usize, pixel: RGBW) {
        let idx = idx * 4;
        self.0[idx] = pixel.red;
        self.0[idx + 1] = pixel.green;
        self.0[idx * 4] = pixel.blue;
        self.0[idx * 4] = pixel.white;
    }
}

impl fmt::Display for DXMPacket {
    fn fmt(&self, fm: &mut fmt::Formatter) -> fmt::Result {
        for line in hexdump::hexdump_iter(&self.0) {
            writeln!(fm, "{}", line)?;
        }
        Ok(())
    }
}

pub struct ArtnetPacket {
    buffer: Vec<DXMPacket>,
}

impl fmt::Display for ArtnetPacket {
    fn fmt(&self, fm: &mut fmt::Formatter) -> fmt::Result {
        for (idx, univer) in self.buffer.iter().enumerate() {
            write!(
                fm,
                "DXM{}, Univer: {}, Buff size: {}\n{}",
                univer.0.len() / 4,
                idx,
                univer.0.len(),
                univer
            )?;
        }
        writeln!(fm, "\n -- \n")
    }
}

///
/// The client performe request to OLA backend but also call process_hook
/// with every arnetPacket sended to OLA, this for debuging prupose
///
pub struct Client<F: (FnMut(&ArtnetPacket))> {
    packet: ArtnetPacket,
    matrix: AddrMap,
    process_hook: F,
}

impl ArtnetPacket {
    fn new(addr: &AddrMap) -> Self {
        Self {
            buffer: (0..=addr.nbr_univer)
                .map(|_| {
                    let sz = addr.univer_size * 4;
                    let mut buf = Vec::with_capacity(sz);
                    unsafe { buf.set_len(sz) };
                    buf.iter_mut().for_each(|e| *e = 0);
                    DXMPacket(buf)
                })
                .collect(),
        }
    }

    fn apply_screen_buffer(&mut self, buffer: &ScreenBuffer, addr: &AddrMap) -> &mut Self {
        for (index, pixel) in buffer.0.iter().enumerate() {
            // Get row from buffer index
            let y = index / addr.width;
            // Get cloumn from buffer index
            let x = index - (y * addr.width);
            // Get led address (tuple of (univer, address)) from address map
            let addr = addr[(x, y)];
            // Apply the pixel to the coresponding dmx packet/arnet univer
            self.buffer[addr.univer].set_pixel(addr.address, *pixel);
        }
        self
    }
}

impl<F: (FnMut(&ArtnetPacket))> Client<F> {
    pub fn new(map: Mapping, process_hook: F) -> Result<Self, GError> {
        if map.row > map.dmx_size {
            return Err(GError::WrongConfig(
                "Mapping::row must be smaller than Mapping::dmx_size",
            ));
        }
        // Return a new client instance and generate led addresses mapping
        // TODO open OLA session
        let matrix = AddrMap::from_mapping(&map);
        let packet = ArtnetPacket::new(&matrix);
        Ok(Self {
            matrix,
            packet,
            process_hook,
        })
    }

    pub fn apply_buffer(&mut self, buffer: &ScreenBuffer) {
        self.packet.apply_screen_buffer(buffer, &self.matrix);
        (self.process_hook)(&self.packet)
    }
}
