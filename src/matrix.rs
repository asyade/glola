use crate::options::*;
use std::cmp::{max, min};
use std::fmt;
use termion::color;

/// Made from a `Mapping` (two dimensional matrix of [x,y] -> dxm address/univer id)
/// and options this struct contains all informations needed to convert image buffer to arnet buffer
#[derive(Debug, From, Into, Clone)]
pub struct AddrMap {
    pub opt: MappingOptExt,
    pub addr: Mapping,
}

/// A "Mapping" is two dieemensional matrix that contains pixel address for a given X,Y
type Mapping = Vec<Vec<PixelAddr>>;

/// Used as internal configuration this struct should be generated from
#[derive(Debug, Clone)]
pub struct MappingOptExt {
    pub height: usize,
    pub width: usize,
    pub univer_width: usize,
    pub univer_height: usize,
    pub univer_per_column: usize,
    pub univer_per_row: usize,
    pub color_mode: ColorMode,
    pub displacement: Displacement,
    pub direction: Direction,
    pub orientation: Orientation,
    pub pixel_size: usize,
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
        let chunk: Vec<Vec<PixelAddr>> = (0..opt.univer_width)
            .map(|x| {
                (0..opt.univer_height)
                    .map(|y| PixelAddr {
                        address: (y + (opt.univer_height * x)) * opt.pixel_size,
                        univer: 0,
                    })
                    .collect()
            })
            .collect();
        // You can match more displasement type here
        let chunk = Self::displacement_zig_zag(chunk);
        let chunk = Self::orientation(chunk, opt.orientation);
        let mapping = (0..opt.univer_per_row * opt.univer_per_column)
            .map(|univer| {
                let mut new_chunk = chunk.clone();
                new_chunk
                    .iter_mut()
                    .flatten()
                    .for_each(|pixel| pixel.univer = univer);
                new_chunk
            })
            .collect();
        Self(mapping, opt)
    }

    fn orientation(
        mut chunk: Vec<Vec<PixelAddr>>,
        orientation: Orientation,
    ) -> Vec<Vec<PixelAddr>> {
        match orientation {
            Orientation::TopLeft => chunk,
            Orientation::TopRight => {
                chunk.reverse();
                chunk
            }
            Orientation::BottomLeft => chunk
                .into_iter()
                .map(|mut e| {
                    e.reverse();
                    e
                })
                .collect(),
            Orientation::BottomRight => {
                chunk.reverse();
                chunk
                    .into_iter()
                    .map(|mut e| {
                        e.reverse();
                        e
                    })
                    .collect()
            }
        }
    }

    fn displacement_zig_zag(chunk: Vec<Vec<PixelAddr>>) -> Vec<Vec<PixelAddr>> {
        chunk
            .into_iter()
            .enumerate()
            .map(|(x, mut line)| {
                if x % 2 != 0 {
                    line.reverse();
                }
                line
            })
            .collect()
    }
}

impl Into<Mapping> for PreMapping {
    fn into(self) -> Mapping {
        let mut mapping = vec![vec![PixelAddr::empty(); self.1.height]; self.1.width];
        for (uid, univer) in self.0.into_iter().enumerate() {
            let univer_y = uid / self.1.univer_per_column;
            let univer_x = uid % self.1.univer_per_column;
            let x_offset_in_matrix = univer_x * self.1.univer_width;
            let y_offset_in_matrix = univer_y * self.1.univer_height;
            for (x, x_item) in univer.into_iter().enumerate() {
                for (y, y_item) in x_item.into_iter().enumerate() {
                    let x = x + x_offset_in_matrix;
                    let y = y + y_offset_in_matrix;
                    if x < self.1.width && y < self.1.height {
                        mapping[x][y] = y_item;
                    }
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
        let nbr_led_per_pixel = opt.color_mode as usize;
        let nbr_led_per_channel = opt.dmx_size / nbr_led_per_pixel;
        let nbr_led_per_channel = nbr_led_per_channel - (nbr_led_per_channel % opt.univer_height);
        // How many width can be handled per univer
        let univer_width = min(nbr_led_per_channel / opt.univer_height, opt.width);
        // How many univers are superposed
        let univer_per_row =
            (opt.height / opt.univer_height) + (opt.height % opt.univer_height != 0) as usize;
        let univer_per_column =
            (opt.width / univer_width) + (opt.width % univer_width != 0) as usize;
        if opt.height % opt.univer_height != 0 {
            println!("Warning: univer height is not well steup, last univer height can't equal other univer height this can result of unedifned behaviour");
        }
        Self {
            width: opt.width,
            height: opt.height,
            univer_width,
            univer_height: opt.univer_height,
            univer_per_column,
            univer_per_row,
            color_mode: opt.color_mode,
            displacement: opt.displacement,
            direction: opt.direction,
            orientation: opt.orientation,
            pixel_size: nbr_led_per_pixel,
        }
    }
}

/// An artnet map is a two dimensions matrix that contais uiver/address tuple for a given x,y

impl AddrMap {
    pub fn from_mapping(opt: MappingOptExt) -> Self {
        let pre = PreMapping::new(opt.clone());
        Self {
            opt,
            addr: pre.into(),
        }
    }
}

pub struct RevAddrMap {
    pub offset: Vec<Vec<usize>>,
}

impl From<AddrMap> for RevAddrMap {
    fn from(map: AddrMap) -> RevAddrMap {
        let mut offset_map = vec![
            vec![0; map.opt.univer_height * map.opt.univer_width];
            map.opt.univer_per_column * map.opt.univer_per_row
        ];
        for x in 0..map.opt.width {
            for y in 0..map.opt.height {
                let addr = map.addr[x][y];
                let univer = &mut offset_map[addr.univer];
                let x_in = x % map.opt.univer_width;
                let y_in = y % map.opt.univer_height;
                univer[addr.address / map.opt.pixel_size] = x_in + (y_in * map.opt.univer_width);
            }
        }
        RevAddrMap { offset: offset_map }
    }
}

#[cfg(test)]
mod tests {}
