///
/// A RGBW buffer represented bt a  &[u32, WIDTH*HEIGHT]
///
pub struct ScreenBuffer<'a>(pub &'a [RGBW]);

///
/// RGBW pixel, can be represented as well as an u32
///
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct RGBW {
    red: u8,
    green: u8,
    blue: u8,
    white: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgbw_u32_interop() {
        assert_eq_size!(RGBW, u32);
    }
}
