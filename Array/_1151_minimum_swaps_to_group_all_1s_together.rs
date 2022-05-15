// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together

pub fn min_swaps(data: Vec<i32>) -> i32 {
    let ones = data.iter().filter(|&x| *x == 1).count();
    let mut count_one = 0;
    for i in 0..ones {
        count_one += data[i];
    }
    let mut max_ones_in_sliding = count_one;
    for end in ones..data.len() {
        count_one += data[end] - data[end - ones];
        max_ones_in_sliding = max_ones_in_sliding.max(count_one);
    }

    return (ones - max_ones_in_sliding as usize) as i32;
}

#[test]
pub fn test() {
    assert_eq!(min_swaps(vec![1, 0, 1, 0, 1]), 1);
}
