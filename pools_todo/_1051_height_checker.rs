// https://leetcode.com/problems/height-checker
// 
// A school is trying to take an annual photo of all the students. The students are asked to stand in a single file line in **non-decreasing order** by height. Let this ordering be represented by the integer array `expected` where `expected[i]` is the expected height of the `i<sup>th</sup>` student in line.
// 
// You are given an integer array `heights` representing the **current order** that the students are standing in. Each `heights[i]` is the height of the `i<sup>th</sup>` student in line (**0-indexed**).
// 
// Return _the **number of indices** where_ `heights[i] != expected[i]`.
// 
// **Example 1:**
// 
// ```
// **Input:** heights = [1,1,4,2,1,3]
// **Output:** 3
// **Explanation:** 
// heights:  [1,1,<u>4</u>,2,<u>1</u>,<u>3</u>]
// expected: [1,1,<u>1</u>,2,<u>3</u>,<u>4</u>]
// Indices 2, 4, and 5 do not match.
// ```
// 
// **Example 2:**
// 
// ```
// **Input:** heights = [5,1,2,3,4]
// **Output:** 5
// **Explanation:**
// heights:  [<u>5</u>,<u>1</u>,<u>2</u>,<u>3</u>,<u>4</u>]
// expected: [<u>1</u>,<u>2</u>,<u>3</u>,<u>4</u>,<u>5</u>]
// All indices do not match.
// ```
// 
// **Example 3:**
// 
// ```
// **Input:** heights = [1,2,3,4,5]
// **Output:** 0
// **Explanation:**
// heights:  [1,2,3,4,5]
// expected: [1,2,3,4,5]
// All indices match.
// ```
// 
// **Constraints:**
// 
// *   `1 <= heights.length <= 100`
// *   `1 <= heights[i] <= 100`

pub fn height_checker(heights: Vec<i32>) -> i32 {

    }

#[test]
pub fn t1() {
}
