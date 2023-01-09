#![allow(dead_code)]

use std::path::Path;

use float_eq::{assert_float_eq, float_eq};

use crate::utils;

pub fn gc(dna: &[u8]) -> f64 {
    let gc_count = dna.iter().filter(|x| **x == b'C' || **x == b'G').count();
    return gc_count as f64 / dna.len() as f64 * 100.0;
}

pub fn max_gc(dnas: &[(Vec<u8>, Vec<u8>)]) -> (&[u8], f64) {
    assert!(dnas.len() > 0);
    let mut max: (&[u8], f64) = (&dnas[0].0, gc(&dnas[0].1));
    for i in 1..dnas.len() {
        let label: &[u8] = &dnas.get(i).unwrap().0;
        let dna: &[u8] = &dnas.get(i).unwrap().1;

        if max.1 < gc(&dna) {
            max = (label, gc(&dna));
        }
    }
    return max;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc() {
        assert_float_eq!(
            gc(
                b"CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGACTGGGAACCTGCGGGCAGTAGGTGGAAT"
            ),
            60.919540,
            abs <= 0.00_1
        );
    }

    #[test]
    fn test_max_gc() {
        let input: Vec<(Vec<u8>, Vec<u8>)> = vec![
            (b">Rosalind_6404".to_vec(),
            b"CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG".to_vec()),
            (b">Rosalind_5959".to_vec(),
            b"CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCTATATCCATTTGTCAGCAGACACGC".to_vec()),
            (b">Rosalind_0808".to_vec(),
            b"CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGACTGGGAACCTGCGGGCAGTAGGTGGAAT".to_vec())
        ];

        let (max_label, max) = max_gc(&input);

        assert_eq!(max_label, b">Rosalind_0808");
        assert_float_eq!(max, 60.919540, abs <= 0.00_1);
    }

    #[test]
    fn test_io_max_gc_sample() {
        let path = Path::new("input/rosalind_gc_sample.txt");
        let input = utils::fasta_read_file(path).unwrap();

        let (max_label, max) = max_gc(&input);

        assert_eq!(max_label, b"Rosalind_0808");
        assert_float_eq!(max, 60.919540, abs <= 0.00_1);
    }

    fn test_io_max_gc() {
        let path = Path::new("input/rosalind_gc.txt");
        let input = utils::fasta_read_file(path).unwrap();

        let (max_label, max) = max_gc(&input);

        assert_eq!(max_label, b"Rosalind_3141");
        assert_float_eq!(max, 51.400862068965516, abs <= 0.00_1);
    }
}
