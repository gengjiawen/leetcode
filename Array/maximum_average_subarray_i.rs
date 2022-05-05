pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut sum = 0;
    for i in 0..k {
        sum += nums[i as usize];
    }
    let mut max = sum;
    for i in k as usize..nums.len() {
        sum = sum + nums[i] - nums[i - k as usize];
        max = max.max(sum);
    }
    return max as f64 / k as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(find_max_average(Vec::from([1, 12, -5, -6, 50, 3]), 4), 12.75);
    }
}
