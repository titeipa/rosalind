#![allow(unused_imports)]
use std::fs;
use std::io;
use std::io::stdout;
use std::io::Write;

mod dna;
mod rna;

fn main() {
    let input: Vec<u8> =
        fs::read("input/rosalind_rna.txt").expect("Should have been able to read the file");

    let result = rna::solve_with_for_inplace(input);

    stdout()
        .write_all(&result)
        .expect("Should be able to write to stdout");
}
