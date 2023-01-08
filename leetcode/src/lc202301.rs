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

/// 2023-01-02  
/// 1801. 积压订单中的订单总数  
/// <https://leetcode.cn/problems/number-of-orders-in-the-backlog/>
impl Solution {
    /// 优先队列模拟 Rust 版本  
    /// 已发题解 <https://leetcode.cn/problems/number-of-orders-in-the-backlog/solution/rust-guan-fang-jie-fa-by-kkbt-c3kn/>
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1000000007;
        use std::collections::BinaryHeap; // price  amount
        let mut buy_orders = BinaryHeap::<(i32, i32)>::new(); //
        let mut sell_orders = BinaryHeap::<(i32, i32)>::new(); // Reverse

        for order in orders {
            let (price, mut amount, order_type) = (order[0], order[1], order[2]);
            if order_type == 0 {
                while amount > 0
                    && !sell_orders.is_empty()
                    && -sell_orders.peek().unwrap().0 <= price
                {
                    let mut sell_order = sell_orders.pop().unwrap();
                    let sell_amount = sell_order.1.min(amount);
                    amount -= sell_amount;
                    sell_order.1 -= sell_amount;
                    if sell_order.1 > 0 {
                        sell_orders.push(sell_order);
                    }
                }
                if amount > 0 {
                    buy_orders.push((price, amount));
                }
            } else {
                while amount > 0 && !buy_orders.is_empty() && buy_orders.peek().unwrap().0 >= price
                {
                    let mut buy_order = buy_orders.pop().unwrap();
                    let buy_amount = buy_order.1.min(amount);
                    amount -= buy_amount;
                    buy_order.1 -= buy_amount;
                    if buy_order.1 > 0 {
                        buy_orders.push(buy_order);
                    }
                }
                if amount > 0 {
                    sell_orders.push((-price, amount)); // for Rust BinaryHeap 只有大顶堆
                }
            }
        }
        let mut total = 0;
        for iter in sell_orders.iter() {
            total = (iter.1 + total) % MOD;
        }
        for iter in buy_orders.iter() {
            total = (iter.1 + total) % MOD;
        }
        return total;
    }
}
#[test]
fn test_get_number_of_backlog_orders() {
    assert_eq!(
        Solution::get_number_of_backlog_orders(vec![
            vec![10, 5, 0],
            vec![15, 2, 1],
            vec![25, 1, 1],
            vec![30, 4, 0]
        ]),
        6
    );
    assert_eq!(
        Solution::get_number_of_backlog_orders(vec![
            vec![7, 1000000000, 1],
            vec![15, 3, 0],
            vec![5, 999999995, 0],
            vec![5, 1, 1]
        ]),
        999999984
    );
    assert_eq!(
        Solution::get_number_of_backlog_orders(vec![
            vec![1, 29, 1],
            vec![22, 7, 1],
            vec![24, 1, 0],
            vec![25, 15, 1],
            vec![18, 8, 1],
            vec![8, 22, 0],
            vec![25, 15, 1],
            vec![30, 1, 1],
            vec![27, 30, 0]
        ]),
        22
    );
    assert_eq!(
        Solution::get_number_of_backlog_orders(vec![
            vec![23, 17, 1],
            vec![18, 27, 0],
            vec![21, 26, 1],
            vec![8, 17, 0],
            vec![13, 22, 1],
            vec![22, 21, 1],
            vec![2, 24, 1],
            vec![5, 7, 0]
        ]),
        69
    );
    // assert_eq!(Solution::solve(),"output");
    // TODO
}

/// 2023-01-03  
/// 2042. 检查句子中的数字是否递增  
/// <https://leetcode.cn/problems/check-if-numbers-are-ascending-in-a-sentence/>
impl Solution {
    /// split_whitespace filter_map windows all `s.parse::<u8>().ok()`
    pub fn are_numbers_ascending(s: String) -> bool {
        s.split_whitespace()
            .filter_map(|s| s.parse::<u8>().ok())
            .collect::<Vec<u8>>()
            .windows(2)
            .all(|x| x[0] < x[1])
    }
}
#[rustfmt::skip]
#[test]
fn test_solve() {
    assert_eq!(
        Solution::are_numbers_ascending("1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string()),
        true
    );
    assert_eq!(
        Solution::are_numbers_ascending("hello world 5 x 5".to_string()),
        false
    );
    assert_eq!(
        Solution::are_numbers_ascending(
            "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string()
        ),
        false
    );
    assert_eq!(
        Solution::are_numbers_ascending("4 5 11 26".to_string()),
        true
    );
    // assert_eq!(Solution::solve(),"output");
    // TODO
}
/// 2023-01-04  
/// 1802. 有界数组中指定下标处的最大值  
/// <https://leetcode.cn/problems/maximum-value-at-a-given-index-in-a-bounded-array/>
impl Solution {
    /// 分类讨论
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let (n, index, max_sum) = (n as usize, index as usize, max_sum as usize);
        let left = index.min(n - index - 1);
        let right = n - 1 - left;
        let s = max_sum - n;
        let s1 = left * left;
        let s2 = right * (right - 1) / 2 + (left + 1) * (2 * right - left) / 2;
        if s <= s1 {
            (s as f64).sqrt() as i32 + 1
        } else if s >= s2 {
            (right + (s - s2) / n + 1) as i32
        } else {
            let t = (4 * left + 1) as f64 / 2f64;
            let delta_x = ((2f64 * (s - s1) as f64 + t * t).sqrt() - t) as i32;
            delta_x + 1 + left as i32
        }
    }
}
#[test]
fn test_max_value() {
    assert_eq!(Solution::max_value(4, 2, 6), 2);
    assert_eq!(Solution::max_value(6, 1, 10), 3);
    assert_eq!(Solution::max_value(3, 2, 18), 7);
    assert_eq!(Solution::max_value(3, 0, 815094800), 271698267);
}
/// 2023-01-06  
/// 2180. 统计各位数字之和为偶数的整数个数  
/// <https://leetcode.cn/problems/count-integers-with-even-digit-sum/>
impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let sum = num.to_string().as_bytes().iter().fold(0i32, |mut sum, x| {
            sum += *x as i32 - 48;
            sum
        }); // 求数位和
        if sum % 2 == 0 {
            return num / 2;
        } else {
            return (num - 1) / 2;
        }
    }
}

/// 2023-01-07  
/// 1658. 将 x 减到 0 的最小操作数  
/// <https://leetcode.cn/problems/minimum-operations-to-reduce-x-to-zero/>
impl Solution {
    /// Code_Description
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut l = nums.len();
        let mut r = 0;
        let mut sum = nums.iter().sum::<i32>();
        let mut ret = -1;
        while r < nums.len() {
            if sum == x && (ret == -1 || l + r < ret as usize) {
                ret = (l + r) as i32;
            }
            if (sum > x && l > 0) || l + r >= nums.len() {
                l -= 1;
                sum -= nums[l];
            } else {
                r += 1;
                sum += nums[nums.len() - r];
            }
        }
        ret
    }
}
#[test]
fn test_min_operations() {
    let ve1 = vec![1, 1, 4, 2, 3];
    assert_eq!(Solution::min_operations(ve1, 5), 2); // 2
    assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1); // -1
    assert_eq!(Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
    assert_eq!(Solution::min_operations(vec![1, 1], 3), -1); // -1
    assert_eq!(Solution::min_operations(vec![1000, 1, 1, 2, 3], 1004), 3); // -1
    assert_eq!(
        Solution::min_operations(
            vec![
                8828, 9581, 49, 9818, 9974, 9869, 9991, 10000, 10000, 10000, 9999, 9993, 9904,
                8819, 1231, 6309
            ],
            134365
        ),
        16
    );
}
/// 2023-01-08  
/// 2185. 统计包含给定前缀的字符串  
/// <https://leetcode.cn/problems/counting-words-with-a-given-prefix/>
impl Solution {
    /// 迭代器
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|x| x.starts_with(&pref)).count() as i32
    }
}
#[test]
fn test_prefix_count() {
    dbg!(Solution::prefix_count(
        vec![
            "pay".to_string(),
            "attention".to_string(),
            "practice".to_string(),
            "attend".to_string()
        ],
        "at".to_string()
    ));
}
