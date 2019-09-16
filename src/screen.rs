use super::prelude::*;
use fps_counter::FPSCounter;

pub struct Screen<T: Encoder + Sized> {
    matrix: AddrMap,
    buffer: Vec<u8>,
    output_encoder: T,
    fps: FPSCounter,
}

impl<T: Encoder + Sized> Screen<T> {
    pub fn new(matrix: AddrMap, output_encoder: T) -> Self {
        let width = matrix.opt.width;
        let height = matrix.opt.height;
        Self {
            buffer: vec![0; width * height * matrix.opt.pixel_size],
            matrix,
            output_encoder,
            fps: FPSCounter::new(),
        }
    }

    /// Apply a screen buffer and output it to the encoder
    /// buffer size must match matrix size, return actual FPS
    pub fn apply<'a>(&'a mut self, buffer: &[u8]) -> (usize, &'a [super::dmx::ArtDmx]) {
        assert_eq!(buffer.len(), self.buffer.len());
        unsafe { self.buffer.set_len(0) };
        self.buffer.extend_from_slice(buffer);
        (
            self.fps.tick(),
            self.output_encoder.encode(&self.matrix, &self.buffer),
        )
    }
}
