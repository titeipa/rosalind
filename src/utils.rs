#![allow(dead_code)]

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use phf::phf_set;

const STOP_CODONS: phf::Set<&str> = phf_set! {"UAA", "UAG ", "UGA"};

fn perm_with_replacement(n: u32, k: u32) -> u32 {
    return n.pow(k);
}

trait SliceExt {
    fn trim(&self) -> &Self;
}

impl SliceExt for [u8] {
    fn trim(&self) -> &[u8] {
        fn is_whitespace(c: &u8) -> bool {
            *c == b'\t' || *c == b' ' || *c == b'\n' || *c == b'\r'
        }

        fn is_not_whitespace(c: &u8) -> bool {
            !is_whitespace(c)
        }

        if let Some(first) = self.iter().position(is_not_whitespace) {
            if let Some(last) = self.iter().rposition(is_not_whitespace) {
                &self[first..last + 1]
            } else {
                unreachable!();
            }
        } else {
            &[]
        }
    }
}

pub fn fasta_read_file(file_path: &Path) -> Result<Vec<(Vec<u8>, Vec<u8>)>, io::Error> {
    let file = File::open(&file_path)?;
    let mut file = io::BufReader::new(file);

    let mut fasta_vector: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();

    let mut label: Vec<u8> = Vec::new();
    let mut seq: Vec<u8> = Vec::new();
    loop {
        let mut buf = vec![];
        let num_bytes = file.read_until(b'\n', &mut buf)?;

        if num_bytes == 0 {
            break;
        }
        let trim_buf = buf.trim();

        if trim_buf.len() == 0 {
            continue; // ignore empty lines
        }

        if trim_buf[0] == b'>' {
            if seq.len() > 0 && label.len() > 0 {
                fasta_vector.push((label, seq));
                seq = Vec::new();
            }

            label = trim_buf[1..].into();
        } else {
            seq.extend_from_slice(trim_buf);
        }
    }

    if seq.len() > 0 && label.len() > 0 {
        fasta_vector.push((label, seq));
    }

    Ok(fasta_vector)
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
