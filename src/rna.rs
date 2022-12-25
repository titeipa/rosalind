#![allow(dead_code)]

pub fn solve_with_replace(dna: String) -> String {
    return dna.replace("T", "U");
}

pub fn solve_with_for_inplace(mut dna: Vec<u8>) -> Vec<u8> {
    for i in 0..dna.len() {
        if dna[i] == b'T' {
            dna[i] = b'U';
        }
    }

    return dna;
}
