use crate::options::{LedOrdering, Mapping};
use std::fmt;
use termion::color;

///
/// [univer, address]
///
#[derive(Debug, From, Into, Clone, Copy)]
pub struct LedAddr {
    pub univer: usize,
    pub address: usize,
}

#[derive(Debug, From, Into)]
pub struct AddrMap {
    pub width: usize,
    pub height: usize,
    pub nbr_univer: usize,
    pub univer_size: usize,
    pub addr: Vec<Vec<LedAddr>>,
}

impl std::ops::Index<(usize, usize)> for AddrMap {
    type Output = LedAddr;
    fn index(&self, index: (usize, usize)) -> &LedAddr {
        &self.addr[index.0][index.1]
    }
}

impl fmt::Display for AddrMap {
    fn fmt(&self, wr: &mut fmt::Formatter) -> fmt::Result {
        let mut curr_univer = 9999;
        for y in 0..self.addr[0].len() {
            for x in self.addr.iter() {
                if x[y].univer != curr_univer {
                    curr_univer = x[y].univer;
                }
                if curr_univer % 2 == 0 {
                    write!(wr, "{}{:03} ", color::Fg(color::Blue), x[y].address)?;
                } else {
                    write!(wr, "{}{:03} ", color::Fg(color::Green), x[y].address)?;
                }
            }
            writeln!(wr)?;
        }
        writeln!(wr, "{}", color::Fg(color::Reset))
    }
}

impl AddrMap {
    pub fn from_mapping(map: &Mapping) -> Self {
        // Get nbr of unusable led, assuming each led strip/row start from the bottom and each row can be made of only one univer @kantum ?
        let led_leak = map.dmx_size % map.row;
        // Get nbr of cloum per univers/dxm packet, taking in consideration the led leak
        let cloumn_per_univer = (map.dmx_size - led_leak) / map.row;
        let mut map_addr: Vec<Vec<LedAddr>> = Vec::with_capacity(map.cloumn);
        // Iterate on every cloumn,
        for x in 0..map.cloumn {
            // Get the univer of the current cloumn
            let univer = x / cloumn_per_univer;
            // Get the address of the first led of the current cloumn
            let begin_addr = (x % cloumn_per_univer) * map.row;
            // push every (univer, address) tuple, dont care about LedOrdering we're going to map the result at the end of the loop
            map_addr.push(
                (0..map.row)
                    .map(|e| LedAddr::from((univer, e + begin_addr)))
                    .collect(),
            );
        }
        Self::from_unordered_matrix(map_addr, map)
    }

    fn from_unordered_matrix(map: Vec<Vec<LedAddr>>, opt: &Mapping) -> Self {
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
