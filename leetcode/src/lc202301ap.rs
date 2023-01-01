//! 用于 leetcode 所有问题 非每日一题

use std::vec;
pub struct Solution;
/// 2023-01-01  
/// 4. 寻找两个正序数组的中位数  
/// <https://leetcode.cn/problems/median-of-two-sorted-arrays/>
impl Solution {
    /// 库 chain sort 浪费了两个正序
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut union = nums1.iter().chain(&nums2).collect::<Vec<_>>();
        union.sort();
        let len = union.len();
        if len % 2 == 0 {
            return (union[len / 2 - 1] + union[len / 2]) as f64 / 2.0;
        } else {
            return *union[len / 2] as f64;
        }
    }
}
#[test]
fn test_find_median_sorted_arrays() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.5
    );
}

/// 2023-01-01  
/// 6. Z 字形变换  
/// <https://leetcode.cn/problems/zigzag-conversion/>
impl Solution {
    /// 模拟
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut vec = vec![Vec::new(); num_rows as usize]; // 每行一个 Vec
        let mut now_len_index = 0; // 0 1 2; 1  len=2*num_rows-1
        let mut now_len = Vec::new();
        for i in 0..num_rows {
            now_len.push(i);
        }
        for i in (1..num_rows - 1).rev() {
            now_len.push(i);
        }
        let len = now_len.len(); // (2 * num_rows - 1)
        let s = s.as_bytes();
        for iter in s {
            vec[now_len[now_len_index % len] as usize].push(*iter);
            now_len_index += 1;
        }
        let mut union = "".to_string();
        for iter in vec {
            union = union + &String::from_utf8(iter).unwrap();
        }
        union
    }
}
#[test]
fn test_convert() {
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 3),
        "PAHNAPLSIIGYIR".to_string()
    );
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 4),
        "PINALSIGYAHRPI".to_string()
    );
    assert_eq!(Solution::convert("A".to_string(), 1), "A".to_string());
}
use std::collections::HashSet;
/// 2023-01-01  
/// 15. 三数之和  
/// <https://leetcode.cn/problems/3sum/>
/// 超时
impl Solution {
    /// Code_Description
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = HashSet::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    if nums[i] > 0 && nums[j] > 0 && nums[k] > 0 {
                        continue;
                    } else if nums[i] < 0 && nums[j] < 0 && nums[k] < 0 {
                        continue;
                    } else if nums[i] + nums[j] + nums[k] == 0 {
                        let mut tem = vec![nums[i], nums[j], nums[k]];
                        tem.sort();
                        ans.insert(tem);
                    }
                }
            }
        }
        ans.into_iter().collect()
    }
}
#[test]
fn test() {
    dbg!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
    // assert_eq!(Solution::solve(),"output");
    // TODO
}
/// 2023-01-01  
/// 11. 盛最多水的容器  
/// <https://leetcode.cn/problems/container-with-most-water/>
impl Solution {
    /// 左右指针 遇到短板收窄一格
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let mut ans = 0;
        while left < right {
            if height[left] < height[right] {
                ans = ans.max((right - left) * height[left] as usize);
                left += 1;
            } else {
                ans = ans.max((right - left) * height[right] as usize);
                right -= 1;
            }
        }
        ans as i32
    }
}
#[test]
fn test_max_area() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
}
