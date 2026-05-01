const BOOK_PRICE: f64 = 8.0;

pub fn calculate_price(cart: &[u8]) -> f64 {
    cart.len() as f64 * BOOK_PRICE
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
}
