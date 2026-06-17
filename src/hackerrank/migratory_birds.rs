/// Migratory Birds challenge from HackerRank
/// Finds the most frequently sighted bird type. 
/// If more than one type is sighted that many times, returns the smallest ID.

pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = vec![0; 6]; // Bird types are 1, 2, 3, 4, 5
    for &bird in arr {
        if bird >= 1 && bird <= 5 {
            counts[bird as usize] += 1;
        }
    }

    let mut max_count = 0;
    let mut result_type = 1;

    for bird_type in 1..=5 {
        if counts[bird_type] > max_count {
            max_count = counts[bird_type];
            result_type = bird_type as i32;
        }
    }

    result_type
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds_sample_1() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_migratory_birds_sample_2() {
        let arr = vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];
        assert_eq!(migratory_birds(&arr), 3);
    }

    #[test]
    fn test_migratory_birds_tie() {
        let arr = vec![1, 1, 2, 2];
        assert_eq!(migratory_birds(&arr), 1);
    }

    #[test]
    fn test_migratory_birds_single_type() {
        let arr = vec![5, 5, 5];
        assert_eq!(migratory_birds(&arr), 5);
    }
}
