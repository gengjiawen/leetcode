// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array
//
// Given the array of integers `nums`, you will choose two different indices `i` and `j` of that array. _Return the maximum value of_ `(nums[i]-1)*(nums[j]-1)`.
//
// **Example 1:**
//
// ```
// **Input:** nums = [3,4,5,2]
// **Output:** 12
// **Explanation:** If you choose the indices i=1 and j=2 (indexed from 0), you will get the maximum value, that is, (nums[1]-1)*(nums[2]-1) = (4-1)*(5-1) = 3*4 = 12\.
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [1,5,4,5]
// **Output:** 16
// **Explanation:** Choosing the indices i=1 and j=3 (indexed from 0), you will get the maximum value of (5-1)*(5-1) = 16.
// ```
//
// **Example 3:**
//
// ```
// **Input:** nums = [3,7]
// **Output:** 12
// ```
//
// **Constraints:**
//
// *   `2 <= nums.length <= 500`
// *   `1 <= nums[i] <= 10^3`

pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut sortable = nums.clone();
    sortable.sort();

    return (sortable[nums.len() - 1] - 1) * (sortable[nums.len() - 2] - 1);
}

pub fn max_product_failed_functional_since_rust_sort_not_return_itself(nums: Vec<i32>) -> i32 {
    let mut r = nums.iter().enumerate().collect::<Vec<(usize, &i32)>>();
    r.sort_by(|a, b| b.1.cmp(a.1));
    return r.iter().take(2).map(|x| (x.1 - 1) as i32).product();
}

#[test]
pub fn t1() {
    assert_eq!(max_product(vec![3, 4, 5, 2]), 12);
}
