use std::fs;

mod dna;

fn main() {
    let input = fs::read_to_string("input/rosalind_dna.txt")
        .expect("Should have been able to read the file");

    let result = dna::solve_basic(input);
    println!("{}", result);
}
