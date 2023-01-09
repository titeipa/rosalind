#![allow(unused_imports)]
use std::fs;
use std::io;
use std::io::stdout;
use std::io::Write;

mod dna;
mod fib;
mod gc;
mod hamm;
mod iprb;
mod revc;
mod rna;
mod utils;

fn main() {
    let input: Vec<u8> =
        fs::read("input/rosalind_revc.txt").expect("Should have been able to read the file");

    let result = revc::solve_in_place(input);

    stdout()
        .write_all(&result)
        .expect("Should be able to write to stdout");
}
