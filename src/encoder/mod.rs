use super::prelude::*;

pub mod artnet;
pub mod debug;
pub use artnet::*;
pub use debug::*;

pub trait Encoder {
    fn encode(&mut self, matrix: &AddrMap, buffer: &[u8]);
}
