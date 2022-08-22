// https://leetcode.com/problems/daily-temperatures
//
// Given an array of integers `temperatures` represents the daily temperatures, return _an array_ `answer` _such that_ `answer[i]` _is the number of days you have to wait after the_ `i<sup>th</sup>` _day to get a warmer temperature_. If there is no future day for which this is possible, keep `answer[i] == 0` instead.
//
// **Example 1:**
//
// ```
// **Input:** temperatures = [73,74,75,71,69,72,76,73]
// **Output:** [1,1,4,2,1,1,0,0]
// ```
//
// **Example 2:**
//
// ```
// **Input:** temperatures = [30,40,50,60]
// **Output:** [1,1,1,0]
// ```
//
// **Example 3:**
//
// ```
// **Input:** temperatures = [30,60,90]
// **Output:** [1,1,0]
// ```
//
// **Constraints:**
//
// *   `1 <= temperatures.length <= 10<sup>5</sup>`
// *   `30 <= temperatures[i] <= 100`

pub fn daily_temperatures_monotonic_stack(temperatures: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; temperatures.len()];
    let mut stack = Vec::new();
    for i in 0..temperatures.len() {
        while !stack.is_empty() && temperatures[i] > temperatures[stack[stack.len() - 1]] {
            let prev = stack.pop().unwrap();
            res[prev] = (i - prev) as i32;
        }
        stack.push(i);
    }

    return res;
}

pub fn daily_temperatures_greedy(temperatures: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; temperatures.len()];
    for i in 0..temperatures.len() {
        for j in i + 1..temperatures.len() {
            if temperatures[j] > temperatures[i] {
                res[i] = (j - i) as i32;
                break;
            }
        }
    }
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(
        daily_temperatures_monotonic_stack(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
    assert_eq!(
        daily_temperatures_greedy(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
}
