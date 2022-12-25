#![allow(dead_code)]

fn complement(nucleotide: u8) -> u8 {
    match nucleotide {
        b'A' => b'T',
        b'T' => b'A',
        b'C' => b'G',
        b'G' => b'C',
        _ => nucleotide,
    }
}

pub fn solve_out_of_place(dna: Vec<u8>) -> Vec<u8> {
    let mut revc = Vec::<u8>::with_capacity(dna.len());

    for i in (0..dna.len()).rev() {
        revc.push(complement(dna[i]));
    }

    return revc;
}

pub fn solve_in_place(mut dna: Vec<u8>) -> Vec<u8> {
    let mut i_rev: usize;
    for i in 0..dna.len() / 2 {
        i_rev = (dna.len() - 1) - i;
        let n = dna[i];

        dna[i] = complement(dna[i_rev]);
        dna[i_rev] = complement(n);
    }

    let revc = dna;

    return revc;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let dna: Vec<u8> = "AAAACCCGGT".as_bytes().to_vec();
        let dna2 = dna.clone();
        let dna3 = dna.clone();

        assert_eq!(solve_in_place(dna), solve_out_of_place(dna2));

        assert_eq!(solve_in_place(dna3), "ACCGGGTTTT".as_bytes().to_vec());
    }
}
