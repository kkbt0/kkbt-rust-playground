//! 这个 Crate 用于存储我个人的 leetcode 代码  
//! Gitee Repository <https://gitee.com/kkbt/rust-play-ground>  
//! by [恐咖兵糖](https://www.ftls.xyz)

// pub mod lc202212; // 2022-12
// pub mod lc202212ap; // 2022-12
pub mod lc202301; // 2023-01 
pub mod lc202301ap; // 2023-01 

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
