use super::prelude::*;

pub mod artnet;
use crate::dmx::ArtDmx;
pub use artnet::*;

pub trait Encoder {
    fn encode<'a>(&'a mut self, matrix: &AddrMap, buffer: &[u8]) -> &'a [ArtDmx];
}
