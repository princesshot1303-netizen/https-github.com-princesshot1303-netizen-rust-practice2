/// Grading Students challenge from HackerRank
/// Rounds grades according to specific rules.
pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        if grade < 38 {
            grade
        } else {
            let next_multiple_of_5 = ((grade / 5) + 1) * 5;
            if next_multiple_of_5 - grade < 3 {
                next_multiple_of_5
            } else {
                grade
            }
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students() {
        let input = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(&input), expected);
    }

    #[test]
    fn test_no_rounding_below_38() {
        assert_eq!(grading_students(&[37]), vec![37]);
    }

    #[test]
    fn test_rounding_up() {
        assert_eq!(grading_students(&[84]), vec![85]);
        assert_eq!(grading_students(&[29]), vec![29]);
        assert_eq!(grading_students(&[57]), vec![57]);
    }
}
