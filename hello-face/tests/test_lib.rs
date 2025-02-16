#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_function() {
        assert_eq!(example_function(2), 4);
    }

    #[test]
    fn test_example_function_negative() {
        assert_eq!(example_function(-2), -4);
    }

    #[test]
    fn test_example_function_zero() {
        assert_eq!(example_function(0), 0);
    }

    #[test]
    fn test_example_function_large() {
        assert_eq!(example_function(1000), 2000);
    }

    #[test]
    fn test_example_function_small() {
        assert_eq!(example_function(1), 2);
    }
}

fn example_function(x: i32) -> i32 {
    x * 2
}
