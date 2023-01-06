#![allow(dead_code)]

use phf::phf_set;

const STOP_CODONS: phf::Set<&str> = phf_set! {"UAA", "UAG ", "UGA"};

fn perm_with_replacement(n: u32, k: u32) -> u32 {
    return n.pow(k);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(perm_with_replacement(1, 3), 1); // 111
        assert_eq!(perm_with_replacement(2, 3), 8); // 111 112 121 122 211 212 221 222
        assert_eq!(perm_with_replacement(4, 3), 64); // AGC etc.. all codons
    }
}
