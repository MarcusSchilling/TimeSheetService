//For future unit tests outside the specific files
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    #[test]
    fn test_eq() {
        assert_eq!(3, 3);
    }

    #[test]
    fn tests_eq() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(2, 2);
    }
}