#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_cart_costs_nothing() {
        assert_eq!(calculate_price(&[]), 0.0);
    }
}
