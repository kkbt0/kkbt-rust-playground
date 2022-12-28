#[allow(dead_code)]
mod lc202212 {
    struct Solution;
    // 2022-12-26
    // 1759. 统计同构子字符串的数目
    // https://leetcode.cn/problems/count-number-of-homogenous-substrings/
    impl Solution {
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
                    println!("{} {} : {} {}", ch[0], ch[1], cnt, ret);
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
    // 2022-12-27
    // 2027. 转换字符串的最少操作次数
    // https://leetcode.cn/problems/minimum-moves-to-convert-string/
    impl Solution {
        pub fn minimum_moves(s: String) -> i32 {
            // X 88 O 79
            let s = s.as_bytes();
            let mut cnt = 0;
            let mut iter = s.iter();
            loop {
                let x = iter.next();
                if x == None {
                    break;
                }
                if x == Some(&b'X') {
                    cnt += 1;
                    iter.next();
                    iter.next();
                }
            }
            cnt
        }
        // 链接：https://leetcode.cn/problems/minimum-moves-to-convert-string/solution/rust-die-dai-qi-by-qweytr_1-vcu8/
        pub fn minimum_moves2(s: String) -> i32 {
            s.bytes()
                .fold((0, 0), |s, x| {
                    if x == b'X' && s.1 <= 0 {
                        (s.0 + 1, 2)
                    } else {
                        (s.0, s.1 - 1)
                    }
                })
                .0
        }
    }
    #[test]
    fn test_minimum_moves() {
        assert_eq!(Solution::minimum_moves(String::from("XXX")),1);
        assert_eq!(Solution::minimum_moves(String::from("XXOX")),2);
        assert_eq!(Solution::minimum_moves(String::from("OOOO")),0);
    }
    // 2022-12-28
    // 1750. 删除字符串两端相同字符后的最短长度
    // https://leetcode.cn/problems/minimum-length-of-string-after-deleting-similar-ends/
    impl Solution {
        pub fn minimum_length(s: String) -> i32 {
            let s = s.as_bytes();
            let (mut left, mut right) = (0, s.len()-1);
            while left < right && s[left] == s[right] {
                let same_char = s[left];
                while left <= right && s[left] == same_char {
                    left += 1;
                }
                while left<=right && s[right] == same_char {
                    right -= 1;
                }
            }
            right as i32 - left as i32+ 1
        }
    }
    #[test]
    fn test_minimum_length() {
        //dbg!(Solution::minimum_length(String::from("ca")));
        assert_eq!(Solution::minimum_length(String::from("ca")), 2);
        assert_eq!(Solution::minimum_length(String::from("cabaabac")), 0);
        assert_eq!(Solution::minimum_length(String::from("aabccabba")),3);
    }
}
