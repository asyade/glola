use crate::options::{LedOrdering, MappingOpt};
use std::fmt;
use termion::color;

///
/// [univer, address]
///
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

#[derive(Debug, From, Into)]
pub struct AddrMap {
    pub width: usize,
    pub height: usize,
    pub nbr_univer: usize,
    pub univer_size: usize,
    pub addr: Vec<Vec<PixelAddr>>,
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
                    0 => write!(wr, "{}{:03} ",  color::Fg(color::Blue), x[y].address),
                    1 => write!(wr, "{}{:03} ", color::Fg(color::Red), x[y].address),
                    2 => write!(wr, "{}{:03} ", color::Fg(color::Green), x[y].address),
                    3 => write!(wr, "{}{:03} ", color::Fg(color::Yellow), x[y].address),
                    _ => write!(wr, "{}{:03} ", color::Fg(color::White), x[y].address),
                };
            }
            writeln!(wr)?;
        }
        writeln!(wr, "{}", color::Fg(color::Reset))
    }
}

type Vec<Vec<Vec<PixelAddr>>>;
type Mapping = Vec<Vec<PixelAddr>>;

/// An artnet map is a two dimensions matrix that contais uiver/address tuple for a given x,y 

impl AddrMap {

    pub fn from_mapping(map: &MappingOpt) -> Self {
        if map.dmx_size < map.univer_height {
            panic!("Univer height must be lesser or equal to dmx buffer size");
        }
        let nbr_led_per_channel = map.dmx_size / 4;
        let led_leak = nbr_led_per_channel % map.univer_height;
        println!("Warning : chunk height is not well setup, {} led are unusable", led_leak);
        // How many column can be handled per univer
        let univer_width = nbr_led_per_channel / map.univer_height;
        // How many univers are superposed
        let nbr_row_univer = map.row / map.univer_height;
        let nbr_univer_per_line = map.column / univer_width;
        if map.row % map.univer_height != 0 {
            println!("Warning: univer height is not well steup, last univer height can't equal other univer height this can result of unedifned behaviour");
        }
        let nbr_univer = nbr_row_univer * nbr_univer_per_line;
        let mut pre_mapping :PreMapping = vec![vec![vec![PixelAddr::empty(); map.univer_height]; univer_width]; nbr_univer];
        // let mut univers: Vec<Vec<Vec<Vec<PixelAddr>>>> = (0..nbr_column_univer).map(|_| (0..nbr_row_univer).map(|_| vec![]));
        let mut rev_row = false;
        let mut last_univer = 0;
        for x in 0..map.column {
            let cloum_univer_idx = x / univer_width;
            dbg!(cloum_univer_idx);
            for y in 0..map.row {
                let row_univer_id = (y / map.univer_height);
                let univer_offset = row_univer_id * nbr_univer_per_line;
                let current_univer = (x / univer_width) + univer_offset;
                let y_offset_in_univer = y - (map.univer_height * row_univer_id);
                let x_offset_in_univer = x - (univer_width * cloum_univer_idx);
                pre_mapping[current_univer][x_offset_in_univer][y_offset_in_univer] = PixelAddr::new(current_univer, y_offset_in_univer + (x_offset_in_univer * univer_width));
            }
            // for i in 0..nbr_row_univer;
        }
        let mut maping:  =  vec![vec![]; map.column];//(0..map.column).map(|e| vec![]).collect();

        Self{
            univer_size: map.dmx_size,
            nbr_univer,
            width: map.column,
            height: map.row,
            addr: maping,
        }
    }
}

#[cfg(test)]
mod tests {
    //!
    //! ## NOTE
    //! You can use theses tests to visualise the generated addresses map, ex:
    //! ```bash
    //! cargo test -- --nocapture test_cloum_from_top
    //! ```
    //!
    use crate::matrix::*;
    use crate::options::*;

    fn basic(opt: MappingOpt) {
        let map = AddrMap::from_mapping(&opt);
        eprintln!("{} ", map);
        map.addr.iter().for_each(|e| {
            assert_eq!(e.len(), opt.row);
            assert!(e.len() < opt.dmx_size);
        });
        assert_eq!(map.addr.len(), opt.column);
    }

    #[test]
    fn test_cloum_from_top() {
        basic(MappingOpt {
            ordering: LedOrdering::NextcolumnFromTop,
            dmx_size: 512,
            column: 50,
            row: 50,
        });
    }

    #[test]
    fn test_cloum_from_bottom() {
        basic(MappingOpt {
            ordering: LedOrdering::NextcolumnFromBottom,
            dmx_size: 512,
            column: 50,
            row: 50,
        });
    }
}
