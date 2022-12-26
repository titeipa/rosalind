#![allow(dead_code)]

pub fn solve_recursive(n: u64, k: u64) -> u64 {
    if n <= 2 {
        return 1;
    }

    return solve_recursive(n - 1, k) + k * solve_recursive(n - 2, k);
}

pub fn solve_iterative(n: u64, k: u64) -> u64 {
    if n <= 2 {
        return 1;
    }

    let mut nn: u64;
    let mut n_1: u64 = 1;
    let mut n_2: u64 = 1;
    for _i in 3..n + 1 {
        nn = n_1 + n_2 * k;
        n_2 = n_1;
        n_1 = nn;
    }
    return n_1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve_recursive(5, 3), 19);
        assert_eq!(solve_iterative(5, 3), 19);
    }

    #[test]
    fn test_solve_downloaded_dataset() {
        assert_eq!(solve_recursive(32, 2), 1431655765);
        assert_eq!(solve_iterative(32, 2), 1431655765);
    }
}
