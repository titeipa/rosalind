#![allow(dead_code)]

use std::fmt;

#[derive(Copy, Clone)]
struct PopulationSnapshot {
    young: u128,  // cannot reproduce
    mature: u128, // includes the dying ones
    dying: u128,  // still alive, will reproduce one more time and not count for next month's census
}

impl PopulationSnapshot {
    fn total(&self) -> u128 {
        return self.young + self.mature;
    }
}

impl fmt::Display for PopulationSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "young: {}, mature: {}, dying: {}",
            self.young, self.mature, self.dying
        )
    }
}

pub fn solve(n: usize, m: usize) -> u128 {
    let mut pops: [PopulationSnapshot; 100] = [PopulationSnapshot {
        young: 0,
        mature: 0,
        dying: 0,
    }; 100];

    pops[0].young = 1;

    for i in 1..n {
        let prev: &PopulationSnapshot = &pops[i - 1];

        let new_pop = PopulationSnapshot {
            young: prev.mature,
            mature: prev.mature - prev.dying + prev.young,
            dying: if i >= (m - 1) {
                pops[i - (m - 1)].young
            } else {
                0
            },
        };
        pops[i] = new_pop;
    }

    return pops[n - 1].total();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(solve(6, 3), 4);
        assert_eq!(solve(70, 10), 150851612688734);
        assert_eq!(solve(99, 16), 215182717852492203481);
        assert_eq!(solve(100, 4), 24382819596721629);
        assert_eq!(solve(87, 16), 669908100703859665);
    }
}
