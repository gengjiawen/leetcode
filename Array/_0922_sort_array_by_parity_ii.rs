// https://leetcode.com/problems/sort-array-by-parity-ii
//
// Given an array of integers `nums`, half of the integers in `nums` are **odd**, and the other half are **even**.
//
// Sort the array so that whenever `nums[i]` is odd, `i` is **odd**, and whenever `nums[i]` is even, `i` is **even**.
//
// Return _any answer array that satisfies this condition_.
//
// **Example 1:**
//
// ```
// **Input:** nums = [4,2,5,7]
// **Output:** [4,5,2,7]
// **Explanation:** [4,7,2,5], [2,5,4,7], [2,7,4,5] would also have been accepted.
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [2,3]
// **Output:** [2,3]
// ```
//
// **Constraints:**
//
// *   `2 <= nums.length <= 2 * 10<sup>4</sup>`
// *   `nums.length` is even.
// *   Half of the integers in `nums` are even.
// *   `0 <= nums[i] <= 1000`
//
// **Follow Up:** Could you solve it in-place?

pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
    let mut even = vec![];
    let mut odd = vec![];
    for num in nums {
        if num % 2 == 0 {
            even.push(num);
        } else {
            odd.push(num);
        }
    }
    return even
        .into_iter()
        .zip(odd.into_iter())
        .map(|(e, o)| vec![e, o])
        .flatten()
        .collect();
}

#[test]
pub fn t1() {
    assert_eq!(sort_array_by_parity_ii(vec![4, 2, 5, 7]), vec![4, 5, 2, 7]);
}
