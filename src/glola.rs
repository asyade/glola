// {
// "orientation": Top-Left,
// "Displacement": snake,
// "Direction": Vertical,
// "universe": 0,
// "dmx_size": 256,
// "colors": rgbw,
// "columns": 5000,
// "rows": 5000
// }

pub enum ColorMode {
    RGBA,
    RGB,
}

pub enum Orientation {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

pub enum Displacement {
    Snake,
    ZigZag,
}

pub enum Direction {
    Horizontal,
    Vertical,
}

pub struct UniverAddr(u32);

pub struct Matrix {
    width: usize,
    height: usize,
    univer: UniverAddr,
    orientation: Orientation,
    direction: Direction,
    displacement: Displacement,
}

pub struct MatrixChunk {
    matrix: Matrix,
    pos_x: usize,
    pos_y: usize,
}

pub struct MatrixSet {
    chunks: MatrixChunk,
    height: usize,
    width: usize,
    chunk_height: usize,
    chunk_width: usize,
}
