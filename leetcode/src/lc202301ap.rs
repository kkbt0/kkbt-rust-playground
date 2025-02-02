//! 2023-01  用于 leetcode 所有问题 非每日一题  

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
use std::collections::{HashMap, HashSet};
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
/// 2023-01-01  
/// 17. 电话号码的字母组合  
/// <https://leetcode.cn/problems/letter-combinations-of-a-phone-number/>
impl Solution {
    /// 全排列  
    /// 已发布题解 <https://leetcode.cn/problems/letter-combinations-of-a-phone-number/solution/by-kkbt-fu3f/>
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        // let ans = Vec::new();
        #[rustfmt::skip]
        let data = [
            vec![String::from("a"),String::from("b"),String::from("c")], // 2
            vec![String::from("d"),String::from("e"),String::from("f")],
            vec![String::from("g"),String::from("h"),String::from("i")],
            vec![String::from("j"),String::from("k"),String::from("l")],
            vec![String::from("m"),String::from("n"),String::from("o")],
            vec![String::from("p"),String::from("q"),String::from("r"),String::from("s")], // 7
            vec![String::from("t"),String::from("u"),String::from("v")],
            vec![String::from("w"),String::from("x"),String::from("y"),String::from("z")], // 9
            ];
        digits.as_bytes().windows(2).fold(
            data[digits.as_bytes()[0] as usize - 50].clone(),
            |vec, index| Self::permutations(&vec, &data[index[1] as usize - 50]),
        )
    }
    fn permutations(list1: &Vec<String>, list2: &Vec<String>) -> Vec<String> {
        let mut ans = Vec::new();
        for i in list1.iter() {
            for j in list2.iter() {
                ans.push(format!("{}{}", i, j));
            }
        }
        ans
    }
}
#[test]
fn test_letter_combinations() {
    dbg!(Solution::letter_combinations("23".to_string()));
    dbg!(Solution::letter_combinations("".to_string()));
    dbg!(Solution::letter_combinations("2".to_string()));
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
/// 2023-01-08  
/// 面试题 04.05. 合法二叉搜索树  
/// <https://leetcode.cn/problems/legal-binary-search-tree-lcci/>
impl Solution {
    /// 中序遍历 判断是否单调增  
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut ans = Vec::new();
        let mut stack = Vec::new();
        let mut root = root;
        while !stack.is_empty() || root.is_some() {
            while let Some(node) = root {
                root = node.borrow_mut().left.take();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                ans.push(node.borrow().val);
                root = node.borrow_mut().right.take();
            }
        }
        ans.windows(2).all(|x| x[0] < x[1])
    }
}
/// 2023-01-08  
/// 1357. 每隔 n 个顾客打折  
/// <https://leetcode.cn/problems/apply-discount-every-n-orders/>
struct Cashier {
    n: i32,
    now_customer: i32,
    discount: i32,
    price: std::collections::HashMap<i32, i32>,
}
#[allow(dead_code)]
impl Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        Self {
            n,
            now_customer: 0,
            discount,
            price: products.into_iter().zip(prices.into_iter()).collect(),
        }
    }
    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        self.now_customer += 1;
        let pay = product
            .iter()
            .zip(amount.iter())
            .fold(0, |acc, (product_id, this_amo)| {
                acc + self.price[&product_id] * this_amo
            });
        if self.now_customer % self.n == 0 {
            (pay as f64) * (1.0 - self.discount as f64 / 100.0)
        } else {
            pay as f64
        }
    }
}

/// 2023-01-08  
/// 1694. 重新格式化电话号码  
/// <https://leetcode.cn/problems/reformat-phone-number/>
impl Solution {
    /// -1 情况移动 -  
    /// 模拟
    pub fn reformat_number(number: String) -> String {
        let tel_num = number
            .as_bytes()
            .iter()
            .filter(|x| (x != &&b' ') && (x != &&b'-'))
            .collect::<Vec<_>>();
        let mut ans = vec![*tel_num[0]];
        for i in 1..tel_num.len() {
            if i % 3 == 0 {
                ans.push(b'-');
            }
            ans.push(*tel_num[i]);
        }
        // -1 情况移动 -
        let len = ans.len();
        if len >= 4 && ans[len - 2] == b'-' {
            ans.swap(len - 2, len - 3);
        }
        String::from_utf8(ans).unwrap()
    }
}
#[test]
fn test_reformat_number() {
    assert_eq!(
        Solution::reformat_number("1-23-45 6".to_string()),
        "123-456".to_string()
    );
    assert_eq!(
        Solution::reformat_number("123 4-567".to_string()),
        "123-45-67".to_string()
    );
    assert_eq!(
        Solution::reformat_number("123 4-5678".to_string()),
        "123-456-78".to_string()
    );
}

/// 2023-01-08  
/// 905. 按奇偶排序数组  
/// <https://leetcode.cn/problems/sort-array-by-parity/>
impl Solution {
    /// 双指针
    /// `nums[i] &1`  和 `nums[i] %2` 是等效的 看起来前者执行快但是内存消耗会大一点  
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let (mut i, mut j) = (0usize, nums.len() - 1);
        while i < j {
            while i < j && nums[i] & 1 == 0 {
                i += 1;
            }
            while i < j && nums[j] & 1 == 1 {
                j -= 1;
            }
            nums.swap(i, j);
        }
        nums
    }
}
#[test]
fn test_sort_array_by_parity() {
    dbg!(Solution::sort_array_by_parity(vec![3, 1, 2, 4]));
    dbg!(Solution::sort_array_by_parity(vec![0, 2]));
    dbg!(Solution::sort_array_by_parity(vec![0, 1, 2]));
    dbg!(Solution::sort_array_by_parity(vec![0, 1]));
    dbg!(Solution::sort_array_by_parity(vec![1, 3]));
}

/// 2023-01-08  
/// 2160. 拆分数位后四位数字的最小和  
/// <https://leetcode.cn/problems/minimum-sum-of-four-digit-number-after-splitting-digits/>
impl Solution {
    pub fn minimum_sum(mut num: i32) -> i32 {
        let mut nums = [0; 4];
        for i in 0..4 {
            nums[i] = num % 10;
            num /= 10;
        }
        nums.sort();
        (nums[0] + nums[1]) * 10 + nums[2] + nums[3]
    }
}
#[test]
fn test_minimum_sum() {
    assert_eq!(Solution::minimum_sum(2932), 52);
}
/// 2023-01-08  
/// 1807. 替换字符串中的括号内容  
/// <https://leetcode.cn/problems/evaluate-the-bracket-pairs-of-a-string/>
impl Solution {
    /// 模拟，先把 knowledge 的对应关系存到 map 里。然后 Vec 存储左右括号位置，用于字符串切片。然后根据位置数据用字符串切片。  
    /// 这道题用 `s = s.replace(&raw_s[left[i]..right[i] + 1], x);` 的字符串 replace 会超时。  
    /// 已发题解 48ms 26.5MB  
    /// ![](https://cdn.ftls.xyz/images/2022/12/Snipaste_2023-01-08_15-35-39.png)  
    /// <https://leetcode.cn/problems/evaluate-the-bracket-pairs-of-a-string/solution/by-kkbt-mxcr/>  
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let map: HashMap<&str, &str> = knowledge
            .iter()
            .map(|x| (x[0].as_str(), x[1].as_str()))
            .collect();
        let (mut left, mut right) = (vec![], vec![]);
        let sbytes = s.as_bytes();
        // 找 () 并存储位置 用于字符串切片
        for i in 0..s.len() {
            if sbytes[i] == b'(' {
                left.push(i);
            } else if sbytes[i] == b')' {
                right.push(i);
            }
        }
        let mut start = 0;
        let mut ans = vec![];
        for i in 0..left.len() {
            ans.push(&s[start..left[i]]);
            // map.get("text") 无括号
            if let Some(x) = map.get(&s[left[i] + 1..right[i]]) {
                ans.push(x);
            } else {
                ans.push("?");
            }
            start = right[i] + 1;
        }
        ans.push(&s[start..s.len()]);
        ans.concat()
    }
    /// 判题机 不支持 HashMap::from_iter  超时
    pub fn evaluate2(mut s: String, knowledge: Vec<Vec<String>>) -> String {
        let map: HashMap<&str, &str> = knowledge
            .iter()
            .map(|x| (x[0].as_str(), x[1].as_str()))
            .collect();
        let binding = s.clone();
        let raw_s = binding;
        let (mut left, mut right) = (vec![], vec![]);
        // 找 () 并存储位置 用于字符串切片
        for i in 0..raw_s.len() {
            let c = raw_s.as_bytes()[i];
            if c == b'(' {
                left.push(i);
            } else if c == b')' {
                right.push(i);
            }
        }
        for i in 0..left.len() {
            // map.get("text") 无括号 s.replace 有括号
            if let Some(x) = map.get(&raw_s[left[i] + 1..right[i]]) {
                s = s.replace(&raw_s[left[i]..right[i] + 1], x);
            } else {
                s = s.replace(&raw_s[left[i]..right[i] + 1], "?");
            }
        }
        s
    }
}
#[test]
fn test_solve() {
    assert_eq!(
        Solution::evaluate(
            "(name)is(age)yearsold".to_string(),
            vec![
                vec!["name".to_string(), "bob".to_string()],
                vec!["age".to_string(), "two".to_string()]
            ]
        ),
        "bobistwoyearsold"
    );
    assert_eq!(
        Solution::evaluate(
            "hi(name)".to_string(),
            vec![vec!["a".to_string(), "b".to_string()]]
        ),
        "hi?"
    );
}

/// 2023-01-08  
/// 2437. 有效时间的数目  
/// <https://leetcode.cn/problems/number-of-valid-clock-times/>
impl Solution {
    /// 0-2 0-3/0-9 0-5 0-9
    pub fn count_time(time: String) -> i32 {
        let time = time.chars().collect::<Vec<_>>();
        let hour = match (time[0], time[1]) {
            ('?', '?') => 24,
            ('?', '0'..='3') => 3,
            ('?', '4'..='9') => 2,
            ('0' | '1', '?') => 10,
            ('2', '?') => 4,
            _ => 1,
        };
        let minute = match (time[3], time[4]) {
            ('?', '?') => 60,
            ('?', _) => 6,
            (_, '?') => 10,
            _ => 1,
        };
        hour * minute
    }
}
#[test]
fn test_count_time() {
    assert_eq!(Solution::count_time(String::from("?5:00")), 2);
    assert_eq!(Solution::count_time(String::from("0?:0?")), 100);
    assert_eq!(Solution::count_time(String::from("??:??")), 1440);
    assert_eq!(Solution::count_time(String::from("?2:16")), 3);
    assert_eq!(Solution::count_time(String::from("?4:22")), 2);
}

/// 2023-01-08  
/// 342. 4的幂  
/// <https://leetcode.cn/problems/power-of-four/>
impl Solution {
    /// 如果 n 是 4 的幂，那么它可以表示成 4^x 的形式，除以 3 的余数一定为 1  
    /// 如果 n 是 2 的幂却不是 4 的幂，那么它可以表示成 4^x * 2 此时它除以 3 的余数一定为 2。  
    /// 因此可以通过 n 除以 3 的余数是否为 1 来判断 n 是否是 4 的幂。  
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0 && n % 3 == 1
    }
    pub fn is_power_of_four2(n: i32) -> bool { // 0xaaaaaaaa 0x2aaaaaaa
        n > 0 && (n & (n - 1)) == 0 && (n & 0x2aaaaaaa ) == 0
    }
}
#[test]
fn test_is_power_of_four() {
    assert_eq!(Solution::is_power_of_four2(4), true);
    assert_eq!(Solution::is_power_of_four(5), false);
}
