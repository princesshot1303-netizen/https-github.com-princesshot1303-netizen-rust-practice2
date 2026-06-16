/// Number Line Jumps (Kangaroo) challenge from HackerRank
/// Determines if two kangaroos will land on the same spot at the same time.
pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        if x1 == x2 {
            "YES".to_string()
        } else {
            "NO".to_string()
        }
    } else {
        // (x2 - x1) % (v1 - v2) == 0 means they will meet at some integer step
        if (x2 - x1) % (v1 - v2) == 0 {
            "YES".to_string()
        } else {
            "NO".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo_yes() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES".to_string());
    }

    #[test]
    fn test_kangaroo_no() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO".to_string());
    }

    #[test]
    fn test_kangaroo_same_velocity_different_start() {
        assert_eq!(kangaroo(0, 2, 5, 2), "NO".to_string());
    }

    #[test]
    fn test_kangaroo_large_input() {
        assert_eq!(kangaroo(21, 6, 47, 3), "NO".to_string());
    }
}
