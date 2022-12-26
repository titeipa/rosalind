#![allow(dead_code)]

// Solution 1

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

// Solution 2
#[derive(Copy, Clone)]
struct Population {
    k: f32,
    n: f32,
    m: f32,
}

fn total(pop: &Population) -> f32 {
    return pop.k + pop.n + pop.m;
}

fn prob_k(pop: &mut Population) -> f32 {
    let prob = pop.k / total(pop);
    pop.k = pop.k - 1.0;
    return prob;
}

fn prob_n(pop: &mut Population) -> f32 {
    let prob = pop.n / total(pop);
    pop.n = pop.n - 1.0;
    return prob;
}

fn prob_m(mut pop: &mut Population) -> f32 {
    let prob = pop.m / total(pop);
    pop.m = pop.m - 1.0;
    return prob;
}

fn prob_km(mut pop: Population) -> f32 {
    let mut pop2 = pop;
    return prob_k(&mut pop) * prob_m(&mut pop) + prob_m(&mut pop2) * prob_k(&mut pop2);
}

fn prob_kn(mut pop: Population) -> f32 {
    let mut pop2 = pop;
    return prob_k(&mut pop) * prob_n(&mut pop) + prob_n(&mut pop2) * prob_k(&mut pop2);
}

fn prob_kk(mut pop: Population) -> f32 {
    return prob_k(&mut pop) * prob_k(&mut pop);
}

fn prob_mn(mut pop: Population) -> f32 {
    let mut pop2 = pop;
    return prob_m(&mut pop) * prob_n(&mut pop) * 0.5 + prob_n(&mut pop2) * prob_m(&mut pop2) * 0.5;
}

fn prob_mm(mut pop: Population) -> f32 {
    return prob_m(&mut pop) * prob_m(&mut pop) * 0.75;
}

pub fn solve_2(k: u32, m: u32, n: u32) -> f32 {
    return solve_2f(k as f32, m as f32, n as f32);
}

pub fn solve_2f(k: f32, m: f32, n: f32) -> f32 {
    let pop = Population { k, m, n };
    return prob_km(pop) + prob_kn(pop) + prob_kk(pop) + prob_mn(pop) + prob_mm(pop);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve(2, 2, 2), 0.78333336);
        assert_eq!(solve(21, 21, 26), 0.7144425);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve_2(2, 2, 2), 0.78333336);
        assert_eq!(solve_2(21, 21, 26), 0.7144425);
    }

    #[test]
    fn test_population_is_mutable() {
        let mut pop = Population {
            k: 4.0,
            m: 3.0,
            n: 2.0,
        };

        prob_k(&mut pop);

        assert_eq!(pop.k, 3.0);
    }

    #[test]
    fn test_prob_fns() {
        let pop = Population {
            k: 2.0,
            m: 2.0,
            n: 2.0,
        };

        prob_km(pop);

        let prob_n: f32 = pop.n / total(&pop);
        let prob_n_1removed: f32 = pop.n / (total(&pop) - 1.0);
        let prob_k: f32 = pop.k / total(&pop);
        let prob_k_1removed: f32 = pop.k / (total(&pop) - 1.0);
        let prob_kn = prob_k * prob_n_1removed + prob_n * prob_k_1removed;

        assert_eq!(prob_km(pop), prob_kn);
    }
}
