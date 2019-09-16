use super::Encoder;
use crate::prelude::*;

pub struct DebugEncoder {}

impl Encoder for DebugEncoder {
    fn encode(&mut self, matrix: &AddrMap, buffer: &[u8]) {}
}
