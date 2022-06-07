// https://leetcode.com/problems/largest-subarray-length-k

pub fn largest_subarray(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let index = nums[0..(nums.len() - k as usize + 1)]
        .to_vec()
        .iter()
        .enumerate()
        .max_by(|x, y| x.1.cmp(&y.1))
        .unwrap()
        .0;

    return nums[index..(index + k as usize)].to_vec();
}

// brute force functional in one line
pub fn largest_subarray_slow(nums: Vec<i32>, k: i32) -> Vec<i32> {
    return (0..nums.len())
        .collect::<Vec<usize>>()
        .iter()
        .fold(vec![], |mut acc, index| {
            if index + k as usize <= nums.len() {
                let sub = nums[*index..(index + k as usize)].to_vec();
                acc.push(sub);
            }
            acc
        })
        .iter()
        .max_by(|x, y| {
            for i in 0..k as usize {
                if x[i] != y[i] {
                    return x[i].cmp(&y[i]);
                }
            }
            return std::cmp::Ordering::Equal;
        })
        .unwrap()
        .to_vec();
}

#[test]
pub fn t1() {
    assert_eq!(largest_subarray(vec![1, 4, 5, 2, 3], 3), [5, 2, 3]);
    assert_eq!(largest_subarray_slow(vec![1, 4, 5, 2, 3], 3), [5, 2, 3]);
}
