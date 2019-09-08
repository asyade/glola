#[macro_use]
extern crate failure;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate static_assertions;
#[macro_use]
extern crate serde;
extern crate termion;
pub mod client;
pub mod matrix;
pub mod options;
pub mod screen;

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
