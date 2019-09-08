//!
//! `OLA` client wrapper that can map `ScreenBuffer` into DXM packet/arnet univers
//!
use crate::matrix::AddrMap;
use crate::options::Mapping;
use crate::screen::{ScreenBuffer, RGBW};
use crate::GError;

pub struct DXMPacket {}

impl DXMPacket {
    pub fn set_pixel(&self, idx: usize, pixel: RGBW) {}
}

pub struct ArtnetPacket {
    buffer: Vec<DXMPacket>,
}

pub struct Client {
    map: Mapping,
    matrix: AddrMap,
}

impl ArtnetPacket {
    fn new() -> Self {
        unimplemented!()
    }

    fn apply_screen_buffer(&mut self, buffer: &ScreenBuffer, addr: AddrMap) -> &mut Self {
        assert_eq!(
            buffer.0.len(),
            addr.addr.len(),
            "ScreenBuffer::len() and AddrMap::len() differ !"
        );
        for (index, pixel) in buffer.0.iter().enumerate() {
            // Get line from buffer index
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

impl Client {
    pub fn new(map: Mapping) -> Result<Self, GError> {
        if map.line > map.dxm_size {
            return Err(GError::WrongConfig(
                "Mapping::line must be smaller than Mapping::dxm_size",
            ));
        }
        // Return a new client instance and generate led addresses mapping
        // TODO open OLA session
        Ok(Self {
            matrix: AddrMap::from_mapping(&map),
            map,
        })
    }
}
