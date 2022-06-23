// https://leetcode.com/problems/maximum-population-year
//
// You are given a 2D integer array `logs` where each `logs[i] = [birth<sub>i</sub>, death<sub>i</sub>]` indicates the birth and death years of the `i<sup>th</sup>` person.
//
// The **population** of some year `x` is the number of people alive during that year. The `i<sup>th</sup>` person is counted in year `x`'s population if `x` is in the **inclusive** range `[birth<sub>i</sub>, death<sub>i</sub> - 1]`. Note that the person is **not** counted in the year that they die.
//
// Return _the **earliest** year with the **maximum population**_.
//
// **Example 1:**
//
// ```
// **Input:** logs = [[1993,1999],[2000,2010]]
// **Output:** 1993
// **Explanation:** The maximum population is 1, and 1993 is the earliest year with this population.
// ```
//
// **Example 2:**
//
// ```
// **Input:** logs = [[1950,1961],[1960,1971],[1970,1981]]
// **Output:** 1960
// **Explanation:**
// The maximum population is 2, and it had happened in years 1960 and 1970.
// The earlier year between them is 1960.```
//
// **Constraints:**
//
// *   `1 <= logs.length <= 100`
// *   `1950 <= birth<sub>i</sub> < death<sub>i</sub> <= 2050`

pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
    let mut max_year = 1950;
    let mut max_pop = 0;
    for i in 1950..=2050 {
        let mut count = 0;
        for j in 0..logs.len() {
            if i >= logs[j][0] && i < logs[j][1] {
                count += 1;
            }
        }
        if count > max_pop {
            max_pop = count;
            max_year = i;
        }
    }
    return max_year;
}

#[test]
pub fn t1() {
    assert_eq!(
        maximum_population(vec![vec![1950, 1961], vec![1960, 1971], vec![1970, 1981]]),
        1960
    );
}
