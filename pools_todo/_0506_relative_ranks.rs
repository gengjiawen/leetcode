// https://leetcode.com/problems/relative-ranks
// 
// You are given an integer array `score` of size `n`, where `score[i]` is the score of the `i<sup>th</sup>` athlete in a competition. All the scores are guaranteed to be **unique**.
// 
// The athletes are **placed** based on their scores, where the `1<sup>st</sup>` place athlete has the highest score, the `2<sup>nd</sup>` place athlete has the `2<sup>nd</sup>` highest score, and so on. The placement of each athlete determines their rank:
// 
// *   The `1<sup>st</sup>` place athlete's rank is `"Gold Medal"`.
// *   The `2<sup>nd</sup>` place athlete's rank is `"Silver Medal"`.
// *   The `3<sup>rd</sup>` place athlete's rank is `"Bronze Medal"`.
// *   For the `4<sup>th</sup>` place to the `n<sup>th</sup>` place athlete, their rank is their placement number (i.e., the `x<sup>th</sup>` place athlete's rank is `"x"`).
// 
// Return an array `answer` of size `n` where `answer[i]` is the **rank** of the `i<sup>th</sup>` athlete.
// 
// **Example 1:**
// 
// ```
// **Input:** score = [5,4,3,2,1]
// **Output:** ["Gold Medal","Silver Medal","Bronze Medal","4","5"]
// **Explanation:** The placements are [1<sup>st</sup>, 2<sup>nd</sup>, 3<sup>rd</sup>, 4<sup>th</sup>, 5<sup>th</sup>].```
// 
// **Example 2:**
// 
// ```
// **Input:** score = [10,3,8,9,4]
// **Output:** ["Gold Medal","5","Bronze Medal","Silver Medal","4"]
// **Explanation:** The placements are [1<sup>st</sup>, 5<sup>th</sup>, 3<sup>rd</sup>, 2<sup>nd</sup>, 4<sup>th</sup>].
// 
// ```
// 
// **Constraints:**
// 
// *   `n == score.length`
// *   `1 <= n <= 10<sup>4</sup>`
// *   `0 <= score[i] <= 10<sup>6</sup>`
// *   All the values in `score` are **unique**.

pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {

    }

#[test]
pub fn t1() {
}
