#[allow(dead_code)]
mod qa {
    fn red(msg: &str) {
        println!("\x1b[91m{}\x1b[0m", msg);
    }
    struct Solution;
    impl Solution {
        // https://leetcode.cn/problems/count-number-of-homogenous-substrings/
        // 1759. 统计同构子字符串的数目
        pub fn count_homogenous(s: String) -> i32 {
            let mut ans: i64 = 0;
            let mut count = 0;
            let mut last_char = s.chars().last().unwrap();
            for c in s.chars() {
                if c == last_char {
                    count += 1;
                } else {
                    ans += count * (count + 1) / 2;
                    last_char = c;
                    count = 1;
                }
            }
            ans += count * (count + 1) / 2;
            (ans % 1000_000_007) as i32
        }
        // https://leetcode.cn/problems/count-number-of-homogenous-substrings/solution/by-kyushu-e5bb/
        pub fn count_homogenous2(s: String) -> i32 {
            s.as_bytes()
                .windows(2)
                .fold((1, 1), |(cnt, ret), ch| {
                    println!("{} {} : {} {}", ch[0] ,ch[1], cnt, ret);
                    if ch[0] == ch[1] {
                        (cnt + 1, (ret + cnt + 1) % 1_000_000_007)
                    } else {
                        (1, (ret + 1) % 1_000_000_007)
                    }
                })
                .1
        }
    }

    #[test]
    fn test_count_homogenous() {
        // let str = String::from("zzzzz"); // 5 4 3 2 1 // (num+1)*num/2
        assert_eq!(Solution::count_homogenous2(String::from("zzzzz")), 15);
        assert_eq!(Solution::count_homogenous(String::from("abbcccaa")), 13); // a bb ccc aaa
        assert_eq!(Solution::count_homogenous(String::from("xy")), 2);
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
