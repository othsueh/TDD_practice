const BOOK_PRICE: f64 = 8.0;
const DISCOUNTS: [f64; 6] = [0.0, 0.0, 0.05, 0.10, 0.20, 0.25];

pub fn calculate_price(cart: &[u8]) -> f64 {
    let mut counts = build_counts(cart);
    let mut groups: Vec<usize> = Vec::new();
    while counts.iter().any(|&c| c > 0) {
        let group_size = counts.iter().filter(|&&c| c > 0).count();
        groups.push(group_size);
        take_one_of_each(&mut counts);
    }
    optimize_groups(&mut groups);
    groups.iter().map(|&s| group_price(s)).sum()
}

fn optimize_groups(groups: &mut [usize]) {
    // Replace every (5-group, 3-group) pair with two 4-groups — always cheaper
    let fives = groups.iter().filter(|&&s| s == 5).count();
    let threes = groups.iter().filter(|&&s| s == 3).count();
    let swaps = fives.min(threes);
    for _ in 0..swaps {
        *groups.iter_mut().find(|s| **s == 5).unwrap() = 4;
        *groups.iter_mut().find(|s| **s == 3).unwrap() = 4;
    }
}

fn build_counts(cart: &[u8]) -> [u32; 6] {
    let mut counts = [0u32; 6];
    for &book in cart {
        counts[book as usize] += 1;
    }
    counts
}

fn group_price(size: usize) -> f64 {
    size as f64 * BOOK_PRICE * (1.0 - DISCOUNTS[size])
}

fn take_one_of_each(counts: &mut [u32; 6]) {
    for c in counts.iter_mut().filter(|c| **c > 0) {
        *c -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_cart_costs_nothing() {
        assert_eq!(calculate_price(&[]), 0.0);
    }

    #[test]
    fn one_book_costs_8_eur() {
        assert_eq!(calculate_price(&[1]), 8.0);
    }

    #[test]
    fn two_copies_of_same_book_costs_16_eur() {
        assert_eq!(calculate_price(&[1, 1]), 16.0);
    }

    #[test]
    fn two_different_books_get_5_percent_discount() {
        assert_eq!(calculate_price(&[1, 2]), 15.2);
    }

    #[test]
    fn two_same_one_different_splits_into_optimal_groups() {
        // group {1,2} at 5% off + group {1} at 0% = 15.2 + 8.0 = 23.2
        assert_eq!(calculate_price(&[1, 1, 2]), 23.2);
    }

    #[test]
    fn three_different_books_get_10_percent_discount() {
        // group {1,2,3} at 10% off = 3 * 8 * 0.9 = 21.6
        assert_eq!(calculate_price(&[1, 2, 3]), 21.6);
    }

    #[test]
    fn four_different_books_get_20_percent_discount() {
        // group {1,2,3,4} at 20% off = 4 * 8 * 0.8 = 25.6
        assert_eq!(calculate_price(&[1, 2, 3, 4]), 25.6);
    }

    #[test]
    fn five_different_books_get_25_percent_discount() {
        // group {1,2,3,4,5} at 25% off = 5 * 8 * 0.75 = 30.0
        assert_eq!(calculate_price(&[1, 2, 3, 4, 5]), 30.0);
    }

    #[test]
    fn greedy_trap_two_groups_of_four_beats_five_plus_three() {
        // greedy: {1,2,3,4,5} + {1,2,3} = 30.0 + 21.6 = 51.6
        // optimal: {1,2,3,4} + {1,2,3,5} = 25.6 + 25.6 = 51.2
        assert_eq!(calculate_price(&[1, 1, 2, 2, 3, 3, 4, 5]), 51.2);
    }
}
