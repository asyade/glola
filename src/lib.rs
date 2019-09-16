#[macro_use]
extern crate failure;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate static_assertions;
#[macro_use]
extern crate serde;
extern crate fps_counter;
extern crate termion;
pub mod dmx;
pub mod encoder;
pub mod matrix;
pub mod options;
pub mod prelude;
pub mod screen;
use prelude::*;
///
/// Library error (returned by all public method)
///
#[derive(Fail, Debug)]
pub enum GError {
    #[fail(display = "Invalide matrix configuration: {}", 0)]
    WrongConfig(&'static str),
}

impl GError {
    ///
    /// As GLOLA can be used as a C library error can be converted into c_int (with a call to perror in some case)
    ///
    fn into_raw_os_error() -> i32 {
        unimplemented!()
    }
}

pub fn init_arnet_screen(opt: MappingOpt) -> Screen<ArtnetEncoder> {
    let opt: MappingOptExt = opt.into();
    let map = AddrMap::from_mapping(opt.clone());
    let encoder = ArtnetEncoder::new(opt.clone());
    Screen::new(map, encoder)
}
