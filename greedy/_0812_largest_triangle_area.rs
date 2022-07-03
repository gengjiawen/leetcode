// https://leetcode.com/problems/largest-triangle-area
//
// Given an array of points on the **X-Y** plane `points` where `points[i] = [x<sub>i</sub>, y<sub>i</sub>]`, return _the area of the largest triangle that can be formed by any three different points_. Answers within `10<sup>-5</sup>` of the actual answer will be accepted.
//
// **Example 1:**
//
// ![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/04/1027.png)
// ```
// **Input:** points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
// **Output:** 2.00000
// **Explanation:** The five points are shown in the above figure. The red triangle is the largest.
// ```
//
// **Example 2:**
//
// ```
// **Input:** points = [[1,0],[0,0],[0,1]]
// **Output:** 0.50000
// ```
//
// **Constraints:**
//
// *   `3 <= points.length <= 50`
// *   `-50 <= x<sub>i</sub>, y<sub>i</sub> <= 50`
// *   All the given points are **unique**.

pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let mut max = 0.0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            for k in j + 1..points.len() {
                let (x1, y1) = (points[i][0], points[i][1]);
                let (x2, y2) = (points[j][0], points[j][1]);
                let (x3, y3) = (points[k][0], points[k][1]);
                let area = 0.5 * ((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)).abs() as f64;
                if area > max {
                    max = area;
                }
            }
        }
    }
    return max;
}

#[test]
pub fn t1() {
    assert_eq!(
        largest_triangle_area(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![0, 2],
            vec![2, 0]
        ]),
        2.00000
    );
}
