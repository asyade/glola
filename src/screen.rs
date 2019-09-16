use std::fmt;
///
/// A RGBW buffer represented bt a  &[u32, WIDTH*HEIGHT]
///
#[derive(From, Into, Constructor, Debug)]
pub struct ScreenBuffer<'a>(pub &'a [u8]);
