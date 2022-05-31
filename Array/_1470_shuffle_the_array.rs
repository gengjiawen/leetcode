// https://leetcode.com/problems/shuffle-the-array
//
// Given the array `nums` consisting of `2n` elements in the form `[x<sub>1</sub>,x<sub>2</sub>,...,x<sub>n</sub>,y<sub>1</sub>,y<sub>2</sub>,...,y<sub>n</sub>]`.
//
// _Return the array in the form_ `[x<sub>1</sub>,y<sub>1</sub>,x<sub>2</sub>,y<sub>2</sub>,...,x<sub>n</sub>,y<sub>n</sub>]`.
//
// **Example 1:**
//
// ```
// **Input:** nums = [2,5,1,3,4,7], n = 3
// **Output:** [2,3,5,4,1,7]
// **Explanation:** Since x<sub>1</sub>=2, x<sub>2</sub>=5, x<sub>3</sub>=1, y<sub>1</sub>=3, y<sub>2</sub>=4, y<sub>3</sub>=7 then the answer is [2,3,5,4,1,7].
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [1,2,3,4,4,3,2,1], n = 4
// **Output:** [1,4,2,3,3,2,4,1]
// ```
//
// **Example 3:**
//
// ```
// **Input:** nums = [1,1,2,2], n = 2
// **Output:** [1,2,1,2]
// ```
//
// **Constraints:**
//
// *   `1 <= n <= 500`
// *   `nums.length == 2n`
// *   `1 <= nums[i] <= 10^3`

pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut res = vec![0; (2 * n) as usize];
    let mut idx = 0;
    for i in 0..n {
        res[idx] = nums[i as usize];
        res[idx + 1] = nums[(n + i) as usize];
        idx += 2;
    }
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(shuffle(vec![1, 1, 2, 2], 2), [1, 2, 1, 2]);
}
