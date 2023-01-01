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
/// #待解决 超时
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

/// 2023-01-01  
/// 12. 整数转罗马数字  
/// <https://leetcode.cn/problems/integer-to-roman/>
impl Solution {
    /// 1 4 5 9  
    /// 10 40 50 90  
    /// 100 400 500 900  
    /// 1000  
    /// 1 <= num <= 3999  
    /// 判题机 不支持 `String::from_iter()` 不能传入 `Vec<char>` :>  
    /// 不过此题换成 `&str` 就能使用 for 循环了
    pub fn int_to_roman(num: i32) -> String {
        let mut vec = Vec::new();
        let mut num = num;

        let num_1000 = num / 1000;
        for _ in 0..num_1000 {
            vec.push(b'M');
        }
        num -= num_1000 * 1000;

        let num_900 = num / 900;
        for _ in 0..num_900 {
            vec.push(b'C');
            vec.push(b'M');
        }
        num -= num_900 * 900;

        let num_500 = num / 500;
        for _ in 0..num_500 {
            vec.push(b'D');
        }
        num -= num_500 * 500;

        let num_400 = num / 400;
        for _ in 0..num_400 {
            vec.push(b'C');
            vec.push(b'D');
        }
        num -= num_400 * 400;

        let num_100 = num / 100;
        for _ in 0..num_100 {
            vec.push(b'C');
        }
        num -= num_100 * 100;

        let num_90 = num / 90;
        for _ in 0..num_90 {
            vec.push(b'X');
            vec.push(b'C');
        }
        num -= num_90 * 90;

        let num_50 = num / 50;
        for _ in 0..num_50 {
            vec.push(b'L');
        }
        num -= num_50 * 50;

        let num_40 = num / 40;
        for _ in 0..num_40 {
            vec.push(b'X');
            vec.push(b'L');
        }
        num -= num_40 * 40;

        let num_10 = num / 10;
        for _ in 0..num_10 {
            vec.push(b'X');
        }
        num -= num_10 * 10;

        let num_9 = num / 9;
        for _ in 0..num_9 {
            vec.push(b'I');
            vec.push(b'X');
        }
        num -= num_9 * 9;

        let num_5 = num / 5;
        for _ in 0..num_5 {
            vec.push(b'V');
        }
        num -= num_5 * 5;

        let num_4 = num / 4;
        for _ in 0..num_4 {
            vec.push(b'I');
            vec.push(b'V');
        }
        num -= num_4 * 4;

        let num_1 = num / 1;
        for _ in 0..num_1 {
            vec.push(b'I');
        }
        // num -= num_1 * 1;
        String::from_utf8(vec).unwrap()
    }
    /// 作者：sealoong
    /// 链接：<https://leetcode.cn/problems/integer-to-roman/solution/rustan-shi-jin-zhi-tan-xin-chu-li-by-sea-qp2r/>
    /// 来源：力扣（LeetCode）
    pub fn int_to_roman2(num: i32) -> String {
        [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ]
        .into_iter()
        .fold(
            (String::with_capacity(20), num),
            |(s, num), (base, unit)| (s + &unit.repeat((num / base) as usize), num % base),
        )
        .0
    }
}
#[test]
fn test_int_to_roman() {
    assert_eq!(Solution::int_to_roman(3), "III");
    assert_eq!(Solution::int_to_roman(4), "IV");
    assert_eq!(Solution::int_to_roman(9), "IX");
    assert_eq!(Solution::int_to_roman(58), "LVIII");
    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    assert_eq!(Solution::int_to_roman2(1994), "MCMXCIV");
    // TODO
}
