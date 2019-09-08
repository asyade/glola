use serde::{Deserialize, Serialize};
///
/// Used to determinate addressing of leds
///
#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum LedOredering {
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
    pub ordering: LedOredering,
    /// ex: 512
    pub dxm_size: usize,
    // Number of cloumn
    pub cloumn: usize,
    // Number of line per cloumn
    pub line: usize,
}
