const BOOK_PRICE: f64 = 8.0;
const DISCOUNTS: [f64; 6] = [0.0, 0.0, 0.05, 0.10, 0.20, 0.25];

pub fn calculate_price(cart: &[u8]) -> f64 {
    let mut unique: Vec<u8> = Vec::new();
    for &book in cart {
        if !unique.contains(&book) {
            unique.push(book);
        }
    }
    let distinct = unique.len();
    let discount = DISCOUNTS[distinct];
    cart.len() as f64 * BOOK_PRICE * (1.0 - discount)
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
}
