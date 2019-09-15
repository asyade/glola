//!
//! User input data structures
//! ```rust
//! //! ```
//! 

use serde::{Deserialize, Serialize};
///
/// Used to determinate addressing of leds
///
#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum LedOrdering {
    ///
    /// Assume led addressing as follow
    ///  +-+  +-+  +-+
    ///  |1|  |8|  |9|
    ///  +-+  +-+  +-+
    ///  +-+  +-+  +-+
    ///  |2|  |7|  ...
    ///  +-+  +-+  +-+
    ///  +-+  +-+
    ///  |3|  |6|
    ///  +-+  +-+
    ///  +-+  +-+
    ///  |4|  |5|
    ///  +-+  +-+
    ///
    NextcolumnFromBottom,
    ///
    /// Assume led addressing as follow
    ///
    /// +-+  +-+  +-+
    /// |1|  |5|  |9|
    /// +-+  +-+  +-+
    /// +-+  +-+  +-+
    /// |2|  |6|  ...
    /// +-+  +-+  +-+
    /// +-+  +-+
    /// |3|  |7|
    /// +-+  +-+
    /// +-+  +-+
    /// |4|  |8|
    /// +-+  +-+
    ///
    NextcolumnFromTop,
}

///
/// Represente how the led matrix/mapping shoulde be constructed
///
#[derive(Debug, Serialize, Deserialize)]
pub struct MappingOpt {
    pub ordering: LedOrdering,
    /// ex: 512
    pub dmx_size: usize,
    // Number of column
    pub column: usize,
    // Number of row per column
    pub row: usize,
    // Height of individual chunk/univers)  width is determinated b dmx_size
    pub univer_height: usize,
    /// Nbr channel used by a single pixel
    pub channel_per_pixel: usize,
}
