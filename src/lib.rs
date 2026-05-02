const BOOK_PRICE: f64 = 8.0;
const DISCOUNTS: [f64; 6] = [0.0, 0.0, 0.05, 0.10, 0.20, 0.25];

pub fn calculate_price(cart: &[u8]) -> f64 {
    let mut counts = [0u32; 6]; // counts[0..5] for books 1..5
    for &book in cart {
        counts[book as usize] += 1;
    }

    let mut total = 0.0;
    loop {
        // pick one of each distinct book still remaining
        let group_size = counts.iter().filter(|&&c| c > 0).count();
        if group_size == 0 {
            break;
        }
        let discount = DISCOUNTS[group_size];
        total += group_size as f64 * BOOK_PRICE * (1.0 - discount);
        for c in counts.iter_mut().filter(|c| **c > 0) {
            *c -= 1;
        }
    }
    total
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
}
