use crate::options::{LedOrdering, Mapping};
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

impl AddrMap {
    pub fn from_mapping(map: &Mapping) -> Self {
        if map.dmx_size < map.univer_height {
            panic!("Univer height must be lesser or equal to dmx buffer size");
        }
        let nbr_led_per_channel = map.dmx_size / 4;
        let led_leak = nbr_led_per_channel % map.univer_height;
        println!("Warning : chunk height is not well setup, {} led are unusable", led_leak);
        // How many cloumn can be handled per univer
        let nbr_cloumn_per_univer = nbr_led_per_channel / map.univer_height;
        // How many univers are superposed
        let nbr_row_univer = map.row / map.univer_height;
        let nbr_univer_per_line = map.cloumn / nbr_cloumn_per_univer;
        if map.row % map.univer_height != 0 {
            println!("Warning: univer height is not well steup, last univer height can't equal other univer height this can result of unedifned behaviour");
        }
        let mut maping: Vec<Vec<PixelAddr>> = (0..map.cloumn).map(|e| vec![]).collect();
        // let mut univers: Vec<Vec<Vec<Vec<PixelAddr>>>> = (0..nbr_cloumn_univer).map(|_| (0..nbr_row_univer).map(|_| vec![]));
        let mut rev_row = false;
        let mut last_univer = 0;
        for x in 0..map.cloumn {
            let cloum_univer_idx = x / nbr_cloumn_per_univer;
            dbg!(cloum_univer_idx);
            for y in 0..map.row {
                let row_univer_id = (y / map.univer_height);
                let univer_offset = row_univer_id * nbr_univer_per_line;
                let current_univer = (x / nbr_cloumn_per_univer) + univer_offset;
                let y_offset_in_univer = y - (map.univer_height * row_univer_id);
                let x_offset_in_univer = x - (nbr_cloumn_per_univer * cloum_univer_idx);
                // TODO other motifs here
                if ((x_offset_in_univer+1) % 2 == 0) {
                    maping[x].push(PixelAddr::new(current_univer,((map.univer_height - 1) -  y_offset_in_univer) + (x_offset_in_univer * nbr_cloumn_per_univer)));
                } else {
                    maping[x].push(PixelAddr::new(current_univer, y_offset_in_univer + (x_offset_in_univer * nbr_cloumn_per_univer)));
                }
                maping[x][y].address *= map.channel_per_pixel;
                // To here
            }
            // for i in 0..nbr_row_univer;
        }

        Self{
            univer_size: map.dmx_size,
            nbr_univer: nbr_row_univer * nbr_univer_per_line,
            width: map.cloumn,
            height: map.row,
            addr: maping,
        }
    }

    fn from_unordered_matrix(map: Vec<Vec<PixelAddr>>, opt: &Mapping) -> Self {
        let nbr_univer: usize = map.last().unwrap().last().unwrap().univer;
        // If the led ordering is NextCloumnFromBottom that mens we should revers 1/2 cloumn assuming the first led strip start from top
        AddrMap {
            univer_size: opt.dmx_size,
            nbr_univer,
            width: opt.cloumn,
            height: opt.row,
            addr: (if opt.ordering == LedOrdering::NextCloumnFromBottom {
                map.into_iter()
                    .enumerate()
                    .map(|(index, cloumn)| {
                        if index % 2 != 0 {
                            // Reverse the led ordering on impair cloumn
                            cloumn.into_iter().rev().collect()
                        } else {
                            cloumn
                        }
                    })
                    .collect()
            } else {
                // When addressing is NextCloumnFromTop no action is needed
                map
            }),
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

    fn basic(opt: Mapping) {
        let map = AddrMap::from_mapping(&opt);
        eprintln!("{} ", map);
        map.addr.iter().for_each(|e| {
            assert_eq!(e.len(), opt.row);
            assert!(e.len() < opt.dmx_size);
        });
        assert_eq!(map.addr.len(), opt.cloumn);
    }

    #[test]
    fn test_cloum_from_top() {
        basic(Mapping {
            ordering: LedOrdering::NextCloumnFromTop,
            dmx_size: 512,
            cloumn: 50,
            row: 50,
        });
    }

    #[test]
    fn test_cloum_from_bottom() {
        basic(Mapping {
            ordering: LedOrdering::NextCloumnFromBottom,
            dmx_size: 512,
            cloumn: 50,
            row: 50,
        });
    }
}
