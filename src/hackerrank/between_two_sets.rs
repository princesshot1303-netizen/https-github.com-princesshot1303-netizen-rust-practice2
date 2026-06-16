/// Between Two Sets challenge from HackerRank
/// Finds the number of integers that are between two sets.

pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0;
    
    // Find the maximum value in a and minimum in b to define the range
    let start = *a.iter().max().unwrap_or(&1);
    let end = *b.iter().min().unwrap_or(&100);
    
    for x in start..=end {
        // x must be a multiple of all elements in a
        let is_multiple_of_a = a.iter().all(|&val| x % val == 0);
        // All elements in b must be a multiple of x
        let is_factor_of_b = b.iter().all(|&val| val % x == 0);
        
        if is_multiple_of_a && is_factor_of_b {
            count += 1;
        }
    }
    
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between_two_sets_example() {
        let a = vec![2, 6];
        let b = vec![24, 36];
        assert_eq!(get_total_x(&a, &b), 2); // 6 and 12
    }

    #[test]
    fn test_between_two_sets_sample() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3); // 4, 8, 16
    }

    #[test]
    fn test_between_two_sets_single_elements() {
        let a = vec![3];
        let b = vec![12];
        // x=3: 3%3=0, 12%3=0 (Yes)
        // x=6: 6%3=0, 12%6=0 (Yes)
        // x=12: 12%3=0, 12%12=0 (Yes)
        assert_eq!(get_total_x(&a, &b), 3);
    }
}
