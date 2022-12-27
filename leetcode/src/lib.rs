mod lc202212;

#[allow(dead_code)]
mod qa {
    fn red(msg: &str) {
        println!("\x1b[91m{}\x1b[0m", msg);
    }
    // Test Solution
    fn test_add() -> i32 {
        1 + 1
    }
    #[test]
    fn test() {
        assert_eq!(1 + 1, test_add());
    }
}
