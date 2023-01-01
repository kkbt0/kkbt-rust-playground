//! 2023-01  用于每日一题  
use std::collections::HashSet;

/// 通用 Solution
pub struct Solution;
/// 2023-01-01  
/// 2351. 第一个出现两次的字母  
/// <https://leetcode.cn/problems/first-letter-to-appear-twice/>
impl Solution {
    /// HashSet 去重 or dp
    pub fn repeated_character(s: String) -> char {
        let s: Vec<char> = s.chars().collect();
        let mut set = HashSet::new();
        for i in s.iter() {
            if set.contains(&i) {
                return *i;
            } else {
                set.insert(i);
            }
        }
        '0'
    }
}
#[test]
fn test_repeated_character() {
    assert_eq!(Solution::repeated_character("abccbaacz".to_string()), 'c');
    assert_eq!(Solution::repeated_character("abcdd".to_string()), 'd');
    // assert_eq!(Solution::solve(),"output");
    // TODO
}
