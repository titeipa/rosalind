#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashMap;

pub fn solve_basic(dna: String) -> String {
    let (mut A, mut C, mut G, mut T) = (0, 0, 0, 0);

    for nucleotide in dna.chars() {
        if nucleotide == 'A' {
            A += 1;
        } else if nucleotide == 'C' {
            C += 1;
        } else if nucleotide == 'G' {
            G += 1;
        } else if nucleotide == 'T' {
            T += 1;
        }
    }

    return format!("{A} {C} {G} {T}");
}

pub fn solve_hashmap(dna: String) -> String {
    let mut counts = HashMap::new();
    counts.insert('A', 0);
    counts.insert('C', 0);
    counts.insert('G', 0);
    counts.insert('T', 0);

    for nucleotide in dna.chars() {
        match counts.get_mut(&nucleotide) {
            Some(count) => *count = *count + 1,
            None => println!("Invalid input chat {}", nucleotide),
        }
    }

    let mut result: String = "".to_string();
    for (_, count) in &counts {
        result.push_str(format!("{count} ").as_str());
    }

    return result;
}