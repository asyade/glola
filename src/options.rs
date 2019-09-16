//!
//! User input data structures
//! ```rust
//! //! ```
//!

use serde::{Deserialize, Serialize};

/// Used to derterminate number of address used by one pixel and image buffer parsing
#[repr(usize)]
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum ColorMode {
    RGBA = 4,
    RGB = 3,
}

/// Used to determinate position of the first led of each univer
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum Orientation {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum Displacement {
    Snake,
    ZigZag,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

///
/// Represente how the led matrix/mapping shoulde be constructed
///
#[derive(Debug, Serialize, Deserialize)]
pub struct MappingOpt {
    /// ex: 512
    pub dmx_size: usize,
    // Number of width
    pub width: usize,
    // Number of height per width
    pub height: usize,
    // Height of individual chunk/univers)  width is determinated b dmx_size
    pub univer_height: usize,
    /// Color mode, nbr channel per pixel is determinated from this, example: RGBA take 4 address and RGB take 3
    pub color_mode: ColorMode,
    pub displacement: Displacement,
    pub direction: Direction,
    pub orientation: Orientation,
}
