pub fn calculate_price(_cart: &[u8]) -> f64 {
    0.0
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
}
