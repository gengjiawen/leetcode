// https://leetcode.com/problems/defuse-the-bomb
//
// You have a bomb to defuse, and your time is running out! Your informer will provide you with a **circular** array `code` of length of `n` and a key `k`.
//
// To decrypt the code, you must replace every number. All the numbers are replaced **simultaneously**.
//
// *   If `k > 0`, replace the `i<sup>th</sup>` number with the sum of the **next** `k` numbers.
// *   If `k < 0`, replace the `i<sup>th</sup>` number with the sum of the **previous** `k` numbers.
// *   If `k == 0`, replace the `i<sup>th</sup>` number with `0`.
//
// As `code` is circular, the next element of `code[n-1]` is `code[0]`, and the previous element of `code[0]` is `code[n-1]`.
//
// Given the **circular** array `code` and an integer key `k`, return _the decrypted code to defuse the bomb_!
//
// **Example 1:**
//
// ```
// **Input:** code = [5,7,1,4], k = 3
// **Output:** [12,10,16,13]
// **Explanation:** Each number is replaced by the sum of the next 3 numbers. The decrypted code is [7+1+4, 1+4+5, 4+5+7, 5+7+1]. Notice that the numbers wrap around.
// ```
//
// **Example 2:**
//
// ```
// **Input:** code = [1,2,3,4], k = 0
// **Output:** [0,0,0,0]
// **Explanation:** When k is zero, the numbers are replaced by 0\.
// ```
//
// **Example 3:**
//
// ```
// **Input:** code = [2,4,9,3], k = -2
// **Output:** [12,5,6,13]
// **Explanation:** The decrypted code is [3+9, 2+3, 4+2, 9+4]. Notice that the numbers wrap around again. If k is negative, the sum is of the **previous** numbers.
// ```
//
// **Constraints:**
//
// *   `n == code.length`
// *   `1 <= n <= 100`
// *   `1 <= code[i] <= 100`
// *   `-(n - 1) <= k <= n - 1`

pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let mut res = vec![0; code.len()];
    if k == 0 {
        return res;
    }
    let double_code = [&code[..], &code[..]].concat();
    if k > 0 {
        let mut sliding_sum = double_code.iter().take(k as usize).sum::<i32>();
        for i in 0..code.len() {
            sliding_sum = sliding_sum - double_code[i] + double_code[(i + k as usize)];
            res[i] = sliding_sum;
        }
    } else {
        let positive_k = -k;
        let mut sliding_sum = double_code[(code.len() - positive_k as usize - 1)..code.len() - 1]
            .iter()
            .sum::<i32>();
        for i in code.len()..(2 * code.len()) {
            sliding_sum = sliding_sum - double_code[i - positive_k as usize - 1]
                + double_code[(i - 1 as usize)];
            res[i - code.len()] = sliding_sum;
        }
    }
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
    assert_eq!(decrypt(vec![1, 2, 3, 4], 0), vec![0, 0, 0, 0]);
    assert_eq!(decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
}
