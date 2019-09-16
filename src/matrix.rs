use crate::options::{LedOrdering, MappingOpt};
use std::fmt;
use termion::color;

/// Made from a `Mapping` (two dimensional matrix of [x,y] -> dxm address/univer id)
/// and options this struct contains all informations needed to convert image buffer to arnet buffer
#[derive(Debug, From, Into)]
pub struct AddrMap {
    pub opt: MappingOptExt,
    pub addr: Mapping,
}

/// A "Mapping" is two dieemensional matrix that contains pixel address for a given X,Y
type Mapping = Vec<Vec<PixelAddr>>;

/// Used as internal configuration this struct should be generated from
#[derive(Debug, Clone)]
pub struct MappingOptExt {
    height: usize,
    width: usize,
    univer_width: usize,
    univer_height: usize,
    univer_per_width: usize,
    univer_per_height: usize,
}

/// the PreMapping is a tree dimensional matrix of [univer, x in univer, y in univer]
/// a big two dimensional matrix is much more easy to use for image to arnet conversion
/// but calculating pixel displacement, direction and orientation is easyer with the
/// tree dimensional matrix so we're using it as a intermediate matrix that can be
/// converted into a `Mapping` (big two timensional matrix) using `::into`
#[derive(Debug, From, Into, Clone)]
struct PreMapping(Vec<Vec<Vec<PixelAddr>>>, MappingOptExt);

/// Contains the coresponding arnet univer/dxm address of a pixel, a pixel can
/// be made from multiple address (for example, rgba led use 3 of them)
/// the size of a pixel is contained into the `Matrix` cause a matrix is
/// always made of same size pixel
/// * @kantum need review *
#[derive(Debug, From, Into, Clone, Copy, Constructor)]
pub struct PixelAddr {
    pub univer: usize,
    pub address: usize,
}

impl PixelAddr {
    fn empty() -> Self {
        Self {
            univer: 0,
            address: 0,
        }
    }
}

impl std::ops::Index<(usize, usize)> for AddrMap {
    type Output = PixelAddr;
    fn index(&self, index: (usize, usize)) -> &PixelAddr {
        &self.addr[index.0][index.1]
    }
}

impl fmt::Display for AddrMap {
    fn fmt(&self, wr: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.addr[0].len() {
            for x in self.addr.iter() {
                match x[y].univer % 3 {
                    0 => write!(wr, "{}{:03} ", color::Fg(color::Blue), x[y].address)?,
                    1 => write!(wr, "{}{:03} ", color::Fg(color::Red), x[y].address)?,
                    2 => write!(wr, "{}{:03} ", color::Fg(color::Green), x[y].address)?,
                    3 => write!(wr, "{}{:03} ", color::Fg(color::Yellow), x[y].address)?,
                    _ => write!(wr, "{}{:03} ", color::Fg(color::White), x[y].address)?,
                };
            }
            writeln!(wr)?;
        }
        writeln!(wr, "{}", color::Fg(color::Reset))
    }
}

impl PreMapping {
    fn new(opt: MappingOptExt) -> Self {
        dbg!(&opt);
        Self(
            vec![
                vec![vec![PixelAddr::empty(); opt.univer_height]; opt.univer_width];
                opt.univer_per_height * opt.univer_per_width
            ],
            opt,
        )
    }

    /// Set address from x,y, each address begin from the top
    // 01 11 21 31 41 51 61 71
    // 02 12 22 32 42 52 62 72
    // 03 13 23 33 43 53 63 73
    // 04 14 24 34 44 54 64 74
    // 05 15 25 35 45 55 65 75
    // 06 16 26 36 46 56 66 76
    // 07 17 27 37 47 57 67 77
    // 08 18 28 38 48 58 68 78
    // 09 19 29 39 49 59 69 79
    pub fn set_address_from_top(&mut self) {
        for x in 0..self.1.width {
            let cloum_univer_idx = x / self.1.univer_width;
            dbg!(cloum_univer_idx);
            for y in 0..self.1.height {
                let row_univer_id = y / self.1.univer_height;
                let univer_offset = row_univer_id * self.1.univer_per_height;
                let current_univer = (x / self.1.univer_width) + univer_offset;
                let y_offset_in_univer = y - (self.1.univer_height * row_univer_id);
                let x_offset_in_univer = x - (self.1.univer_width * cloum_univer_idx);
                self.set_address(
                    x_offset_in_univer,
                    y_offset_in_univer,
                    current_univer,
                    y_offset_in_univer + (x_offset_in_univer * self.1.univer_width),
                );
            }
            // for i in 0..univer_per_width;
        }
    }

    pub fn set_address(&mut self, x: usize, y: usize, univer: usize, address: usize) {
        self.0[univer][x][y] = PixelAddr::new(univer, address);
    }
}

impl Into<Mapping> for PreMapping {
    fn into(self) -> Mapping {
        let mut mapping = vec![vec![PixelAddr::empty(); self.1.height]; self.1.width];
        for (uid, univer) in self.0.into_iter().enumerate() {
            let univer_y = uid / self.1.univer_per_width;
            let univer_x = uid % self.1.univer_per_width;
            let x_offset_in_matrix = univer_x * self.1.univer_width;
            let y_offset_in_matrix = univer_y * self.1.univer_height;
            dbg!(univer_x, univer_y);
            for (x, x_item) in univer.into_iter().enumerate() {
                for (y, y_item) in x_item.into_iter().enumerate() {
                    mapping[x + x_offset_in_matrix][y + y_offset_in_matrix] = y_item;
                }
            }
        }
        mapping
    }
}

impl From<MappingOpt> for MappingOptExt {
    fn from(opt: MappingOpt) -> Self {
        if opt.dmx_size < opt.univer_height {
            panic!("Univer height must be lesser or equal to dmx buffer size");
        }
        let nbr_led_per_channel = opt.dmx_size / 4;
        let nbr_led_per_channel = nbr_led_per_channel - (nbr_led_per_channel % opt.univer_height);
        // How many column can be handled per univer
        let univer_width = nbr_led_per_channel / opt.univer_height;
        // How many univers are superposed
        let univer_per_height = opt.row / opt.univer_height;
        let univer_per_width = opt.column / univer_width;
        if opt.row % opt.univer_height != 0 {
            println!("Warning: univer height is not well steup, last univer height can't equal other univer height this can result of unedifned behaviour");
        }
        Self {
            width: opt.column,
            height: opt.row,
            univer_width,
            univer_height: opt.univer_height,
            univer_per_width,
            univer_per_height,
        }
    }
}

/// An artnet map is a two dimensions matrix that contais uiver/address tuple for a given x,y

impl AddrMap {
    pub fn from_mapping(opt: MappingOptExt) -> Self {
        let mut pre = PreMapping::new(opt.clone());
        pre.set_address_from_top();
        //TODO match different motifs here
        Self {
            opt,
            addr: pre.into(),
        }
    }
}

#[cfg(test)]
mod tests {}
