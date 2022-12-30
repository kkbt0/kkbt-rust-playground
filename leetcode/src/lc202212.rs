//! 用于每日一题
/// 通用 Solution
#[allow(dead_code)]
pub struct Solution;
/// 2022-12-26  
/// lc 1759. 统计同构子字符串的数目  
/// <https://leetcode.cn/problems/count-number-of-homogenous-substrings/>
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
/// 2022-12-27  
/// lc 2027. 转换字符串的最少操作次数  
/// <https://leetcode.cn/problems/minimum-moves-to-convert-string/>
impl Solution {
    /// 使用 next() 消耗和移动指针
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
    /// 链接：<https://leetcode.cn/problems/minimum-moves-to-convert-string/solution/rust-die-dai-qi-by-qweytr_1-vcu8/>
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
    assert_eq!(Solution::minimum_moves(String::from("XXX")), 1);
    assert_eq!(Solution::minimum_moves(String::from("XXOX")), 2);
    assert_eq!(Solution::minimum_moves(String::from("OOOO")), 0);
}
/// 2022-12-28  
/// lc 1750. 删除字符串两端相同字符后的最短长度  
/// <https://leetcode.cn/problems/minimum-length-of-string-after-deleting-similar-ends/>
impl Solution {
    /// 模拟
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut left, mut right) = (0, s.len() - 1);
        while left < right && s[left] == s[right] {
            let same_char = s[left];
            while left <= right && s[left] == same_char {
                left += 1;
            }
            while left <= right && s[right] == same_char {
                right -= 1;
            }
        }
        right as i32 - left as i32 + 1
    }
}
#[test]
fn test_minimum_length() {
    //dbg!(Solution::minimum_length(String::from("ca")));
    assert_eq!(Solution::minimum_length(String::from("ca")), 2);
    assert_eq!(Solution::minimum_length(String::from("cabaabac")), 0);
    assert_eq!(Solution::minimum_length(String::from("aabccabba")), 3);
}
use std::collections::HashSet;
/// 2022-12-29  
/// lc 2032. 至少在两个数组中出现的值  
/// <https://leetcode.cn/problems/two-out-of-three/>
impl Solution {
    /// 模拟
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        // let set: HashSet<i32> = v.into_iter().collect::<HashSet<i32>>();
        // let set1: HashSet<i32> = v1.iter().map(|x| *x).collect();
        // let hs1:HashSet<i32> = HashSet::from_iter(nums1.into_iter());
        let hs1: HashSet<i32> = nums1.iter().map(|x| *x).collect();
        let hs2: HashSet<i32> = nums2.iter().map(|x| *x).collect();
        let hs3: HashSet<i32> = nums3.iter().map(|x| *x).collect();
        let mut dp: [i32; 105] = [0; 105];
        for i in hs1.iter() {
            dp[*i as usize] += 1;
        }
        for i in hs2.iter() {
            dp[*i as usize] += 1;
        }
        for i in hs3.iter() {
            dp[*i as usize] += 1;
        }
        let mut ans = vec![];
        for i in 0..105 {
            if dp[i] >= 2 {
                ans.push(i as i32);
            }
        }
        ans
    }
}
#[test]
fn test_two_out_of_three() {
    assert_eq!(
        Solution::two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3]),
        vec![2, 3]
    );
    assert_eq!(
        Solution::two_out_of_three(vec![3, 1], vec![2, 3], vec![1, 2]),
        vec![1, 2, 3]
    );
    assert_eq!(
        Solution::two_out_of_three(vec![1, 2, 2], vec![4, 3, 3], vec![5]),
        vec![]
    );
}
use std::collections::BTreeSet;
/// 2022-12-30  
/// 855. 考场就座  
/// <https://leetcode.cn/problems/exam-room/>  
/// 令人迷惑的一道题
pub struct ExamRoom {
    set: BTreeSet<i32>,
    cap: i32,
}
#[allow(dead_code)]
impl ExamRoom {
    fn new(n: i32) -> Self {
        ExamRoom {
            set: BTreeSet::new(),
            cap: n,
        }
    }

    fn seat(&mut self) -> i32 {
        let (mut left, mut right) = (0, self.cap - 1);
        if self.set.len() == 0 {
            self.set.insert(0);
            return 0;
        } else {
            // size <=1
            if self.set.len() == 1 && self.set.contains(&left) {
                self.set.insert(right);
                return right;
            } else if self.set.len() == 1 && self.set.contains(&right) {
                self.set.insert(left);
                return left;
            } else if self.set.len() == 1 {
                for iter in self.set.iter() {
                    if (*iter - left) < (right - *iter) {
                        self.set.insert(right);
                        return right;
                    }
                }
            }
            // size >=2
            let mut pre = 0;
            let mut max_len = 0;
            for (index, val) in self.set.iter().enumerate() {
                // 第一个元素
                if index == 0 {
                    pre = *val;
                    max_len = pre - left;
                    continue;
                } else {
                    // 3 4
                    if *val - pre > max_len {
                        max_len = pre - left;
                        left = pre;
                        right = *val;
                    }
                }
                pre = *val;
            }
            if !self.set.contains(&(self.cap as i32 - 1)) {
                if (self.cap as i32 - 1) - pre > max_len {
                    //
                    // max_len = right - pre;
                    left = pre;
                    right = self.cap as i32 - 1;
                }
            }
        }
        self.set.insert((right + left) / 2 as i32);
        (right + left) / 2 as i32
    }
    fn leave(&mut self, p: i32) {
        self.set.remove(&p);
    }
}
#[test]
fn test_exam_room() {
    let mut room = ExamRoom::new(10);
    dbg!(room.seat()); // 0
    println!("\x1b[91mNow Seat Status: {:?}\x1b[0m", room.set);
    dbg!(room.seat()); // 9
    println!("\x1b[91mNow Seat Status: {:?}\x1b[0m", room.set);
    dbg!(room.seat()); // 4
    println!("\x1b[91mNow Seat Status: {:?}\x1b[0m", room.set);
    dbg!(room.seat()); // 2
    println!("\x1b[91mNow Seat Status: {:?}\x1b[0m", room.set);
    dbg!(room.leave(4)); // ()
    println!("\x1b[91mNow Seat Status: {:?}\x1b[0m", room.set);
    dbg!(room.seat()); // 5
    println!("\x1b[91mNow Seat Status: {:?}\x1b[0m", room.set);
    // 0 _ _ _ 4 _ _ _ _ _ 9 ??? why not 6 is 2
}
