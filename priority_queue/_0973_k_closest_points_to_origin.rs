// https://leetcode.com/problems/k-closest-points-to-origin
//
// Given an array of `points` where `points[i] = [x<sub>i</sub>, y<sub>i</sub>]` represents a point on the **X-Y** plane and an integer `k`, return the `k` closest points to the origin `(0, 0)`.
//
// The distance between two points on the **X-Y** plane is the Euclidean distance (i.e., `√(x<sub>1</sub> - x<sub>2</sub>)<sup>2</sup> + (y<sub>1</sub> - y<sub>2</sub>)<sup>2</sup>`).
//
// You may return the answer in **any order**. The answer is **guaranteed** to be **unique** (except for the order that it is in).
//
// **Example 1:**
//
// ![](https://assets.leetcode.com/uploads/2021/03/03/closestplane1.jpg)
// ```
// **Input:** points = [[1,3],[-2,2]], k = 1
// **Output:** [[-2,2]]
// **Explanation:**
// The distance between (1, 3) and the origin is sqrt(10).
// The distance between (-2, 2) and the origin is sqrt(8).
// Since sqrt(8) < sqrt(10), (-2, 2) is closer to the origin.
// We only want the closest k = 1 points from the origin, so the answer is just [[-2,2]].
// ```
//
// **Example 2:**
//
// ```
// **Input:** points = [[3,3],[5,-1],[-2,4]], k = 2
// **Output:** [[3,3],[-2,4]]
// **Explanation:** The answer [[-2,4],[3,3]] would also be accepted.
// ```
//
// **Constraints:**
//
// *   `1 <= k <= points.length <= 10<sup>4</sup>`
// *   `-10<sup>4</sup> < x<sub>i</sub>, y<sub>i</sub> < 10<sup>4</sup>`

pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut priority_queue = std::collections::BinaryHeap::new();
    for point in points {
        let distance = point[0].pow(2) + point[1].pow(2);
        priority_queue.push((distance, point));
        if priority_queue.len() > k as usize {
            priority_queue.pop();
        }
    }
    return priority_queue.into_iter().map(|(_, point)| point).collect();
}

#[test]
pub fn t1() {
    assert_eq!(
        k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
        vec![vec![-2, 2]]
    );
}
