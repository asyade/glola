use super::prelude::*;
use fps_counter::FPSCounter;

pub struct Screen<T: Encoder + Sized> {
    matrix: AddrMap,
    buffer: Vec<u8>,
    width: usize,
    height: usize,
    output_encoder: T,
    fps: FPSCounter,
}

impl<T: Encoder + Sized> Screen<T> {
    pub fn new(matrix: AddrMap, output_encoder: T) -> Self {
        let width = matrix.opt.width;
        let height = matrix.opt.height;
        Self {
            matrix,
            buffer: vec![0; width * height],
            height,
            width,
            output_encoder,
            fps: FPSCounter::new(),
        }
    }

    /// Apply a screen buffer and output it to the encoder
    /// buffer size must match matrix size, return actual FPS
    pub fn apply(&mut self, buffer: &[u8]) -> usize {
        assert_eq!(buffer.len(), self.buffer.len());
        unsafe { self.buffer.set_len(0) };
        self.buffer.extend_from_slice(buffer);
        self.output_encoder.encode(&self.matrix, &self.buffer);
        self.fps.tick()
    }
}
