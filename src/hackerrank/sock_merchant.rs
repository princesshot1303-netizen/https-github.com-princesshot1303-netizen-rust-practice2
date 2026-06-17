/// Sales by Match (Sock Merchant) challenge from HackerRank
/// Counts the number of matching pairs of socks.

use std::collections::HashMap;

pub fn sock_merchant(n: i32, ar: &[i32]) -> i32 {
    let mut counts = HashMap::new();
    for &sock in ar.iter().take(n as usize) {
        *counts.entry(sock).or_insert(0) += 1;
    }

    let mut pairs = 0;
    for &count in counts.values() {
        pairs += count / 2;
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant_sample_1() {
        let ar = vec![1, 2, 1, 2, 1, 3, 2];
        assert_eq!(sock_merchant(7, &ar), 2);
    }

    #[test]
    fn test_sock_merchant_sample_2() {
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(9, &ar), 3);
    }

    #[test]
    fn test_sock_merchant_no_pairs() {
        let ar = vec![1, 2, 3, 4, 5];
        assert_eq!(sock_merchant(5, &ar), 0);
    }

    #[test]
    fn test_sock_merchant_all_pairs() {
        let ar = vec![1, 1, 2, 2, 3, 3];
        assert_eq!(sock_merchant(6, &ar), 3);
    }
}
