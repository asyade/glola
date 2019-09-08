//!
//! `OLA` client wrapper that can map `ScreenBuffer` into DXM packet/arnet univers
//!
use crate::prelude::{AddrMap, GError, Mapping, ScreenBuffer, RGBW};

pub struct DXMPacket {}

impl DXMPacket {
    #[inline(always)]
    pub fn set_pixel(&self, idx: usize, pixel: RGBW) {}
}

pub struct ArtnetPacket {
    buffer: Vec<DXMPacket>,
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
            buffer: (0..=addr.nbr_univer).map(|_| DXMPacket {}).collect(),
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
