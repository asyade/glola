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
    NextCloumnFromBottom,
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
    NextCloumnFromTop,
}

///
/// Represente how the led matrix/mapping shoulde be constructed
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Mapping {
    pub ordering: LedOrdering,
    /// ex: 512
    pub dmx_size: usize,
    // Number of cloumn
    pub cloumn: usize,
    // Number of row per cloumn
    pub row: usize,
    // Height of individual chunk/univers)  width is determinated b dmx_size
    pub univer_height: usize,
    /// Nbr channel used by a single pixel
    pub channel_per_pixel: usize,
}
