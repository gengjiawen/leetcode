// https://leetcode.com/problems/two-sum-less-than-k

pub fn two_sum_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums_clone = nums.clone();
    nums_clone.sort_unstable();
    let mut res = -1;
    let mut left = 0;
    let mut right = nums_clone.len() - 1;
    while left < right {
        let sum = nums_clone[left] + nums_clone[right];
        if sum < k {
            res = res.max(sum);
            left += 1;
        } else {
            right -= 1;
        }
    }

    return res;
}

#[test]
pub fn t1() {
    assert_eq!(two_sum_less_than_k(vec![34, 23, 1, 24, 75, 33, 54, 8], 60), 58);
}

