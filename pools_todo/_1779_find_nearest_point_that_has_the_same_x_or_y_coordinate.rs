// https://leetcode.com/problems/find-nearest-point-that-has-the-same-x-or-y-coordinate
// 
// You are given two integers, `x` and `y`, which represent your current location on a Cartesian grid: `(x, y)`. You are also given an array `points` where each `points[i] = [a<sub>i</sub>, b<sub>i</sub>]` represents that a point exists at `(a<sub>i</sub>, b<sub>i</sub>)`. A point is **valid** if it shares the same x-coordinate or the same y-coordinate as your location.
// 
// Return _the index **(0-indexed)** of the **valid** point with the smallest **Manhattan distance** from your current location_. If there are multiple, return _the valid point with the **smallest** index_. If there are no valid points, return `-1`.
// 
// The **Manhattan distance** between two points `(x<sub>1</sub>, y<sub>1</sub>)` and `(x<sub>2</sub>, y<sub>2</sub>)` is `abs(x<sub>1</sub> - x<sub>2</sub>) + abs(y<sub>1</sub> - y<sub>2</sub>)`.
// 
// **Example 1:**
// 
// ```
// **Input:** x = 3, y = 4, points = [[1,2],[3,1],[2,4],[2,3],[4,4]]
// **Output:** 2
// **Explanation:** Of all the points, only [3,1], [2,4] and [4,4] are valid. Of the valid points, [2,4] and [4,4] have the smallest Manhattan distance from your current location, with a distance of 1\. [2,4] has the smallest index, so return 2.```
// 
// **Example 2:**
// 
// ```
// **Input:** x = 3, y = 4, points = [[3,4]]
// **Output:** 0
// **Explanation:** The answer is allowed to be on the same location as your current location.```
// 
// **Example 3:**
// 
// ```
// **Input:** x = 3, y = 4, points = [[2,3]]
// **Output:** -1
// **Explanation:** There are no valid points.```
// 
// **Constraints:**
// 
// *   `1 <= points.length <= 10<sup>4</sup>`
// *   `points[i].length == 2`
// *   `1 <= x, y, a<sub>i</sub>, b<sub>i</sub> <= 10<sup>4</sup>`

pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {

    }

#[test]
pub fn t1() {
}
