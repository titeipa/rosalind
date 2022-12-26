#![allow(dead_code)]

pub fn solve(k: u32, m: u32, n: u32) -> f32 {
    return solve_f(k as f32, m as f32, n as f32);
}

pub fn solve_f(k: f32, m: f32, n: f32) -> f32 {
    let npop = k + m + n;

    let prob_k: f32 = k / npop;
    let prob_k_1kremoved: f32 = (k - 1.0) / (npop - 1.0);
    let prob_k_1removed: f32 = k / (npop - 1.0);
    let prob_m: f32 = m / npop;
    let prob_m_1mremoved: f32 = (m - 1.0) / (npop - 1.0);
    let prob_m_1removed: f32 = m / (npop - 1.0);
    let prob_n: f32 = n / npop;
    let prob_n_1removed: f32 = n / (npop - 1.0);

    let prob_kn = prob_k * prob_n_1removed + prob_n * prob_k_1removed;
    let prob_km = prob_k * prob_m_1removed + prob_m * prob_k_1removed;
    let prob_kk = prob_k * prob_k_1kremoved;
    let prob_mn = prob_m * prob_n_1removed * 0.5 + prob_n * prob_m_1removed * 0.5;
    let prob_mm = prob_m * prob_m_1mremoved * 0.75;

    return prob_kn + prob_km + prob_kk + prob_mn + prob_mm;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(2, 2, 2), 0.78333);
    }

    #[test]
    fn test_solv() {
        assert_eq!(solve(21, 21, 26), 0.7144425);
    }
}
