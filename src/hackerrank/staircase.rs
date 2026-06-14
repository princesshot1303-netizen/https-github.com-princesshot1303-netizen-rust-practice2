/// Staircase challenge from HackerRank
/// Prints a right-aligned staircase of size n using '#' symbols.
pub fn staircase(n: i32) -> String {
    let mut result = String::new();
    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        result.push_str(&spaces);
        result.push_str(&hashes);
        if i < n {
            result.push('\n');
        }
    }
    result
}

/// Wrapper for printing to match HackerRank requirements
pub fn print_staircase(n: i32) {
    println!("{}", staircase(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_size_4() {
        let expected = "   #\n  ##\n ###\n####";
        assert_eq!(staircase(4), expected);
    }

    #[test]
    fn test_staircase_size_1() {
        let expected = "#";
        assert_eq!(staircase(1), expected);
    }

    #[test]
    fn test_staircase_size_6() {
        let expected = "     #\n    ##\n   ###\n  ####\n #####\n######";
        assert_eq!(staircase(6), expected);
    }
}
