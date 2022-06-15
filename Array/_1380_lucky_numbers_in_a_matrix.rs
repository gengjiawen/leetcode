// https://leetcode.com/problems/lucky-numbers-in-a-matrix
//
// Given an `m x n` matrix of **distinct** numbers, return _all **lucky numbers** in the matrix in **any** order_.
//
// A **lucky number** is an element of the matrix such that it is the minimum element in its row and maximum in its column.
//
// **Example 1:**
//
// ```
// **Input:** matrix = [[3,7,8],[9,11,13],[15,16,17]]
// **Output:** [15]
// **Explanation:** 15 is the only lucky number since it is the minimum in its row and the maximum in its column.
// ```
//
// **Example 2:**
//
// ```
// **Input:** matrix = [[1,10,4,2],[9,3,8,7],[15,16,17,12]]
// **Output:** [12]
// **Explanation:** 12 is the only lucky number since it is the minimum in its row and the maximum in its column.
// ```
//
// **Example 3:**
//
// ```
// **Input:** matrix = [[7,8],[1,2]]
// **Output:** [7]
// **Explanation:** 7 is the only lucky number since it is the minimum in its row and the maximum in its column.
// ```
//
// **Constraints:**
//
// *   `m == mat.length`
// *   `n == mat[i].length`
// *   `1 <= n, m <= 50`
// *   `1 <= matrix[i][j] <= 10<sup>5</sup>`.
// *   All elements in the matrix are distinct.

pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let (max_in_row, max_index) = matrix
        .iter()
        .map(|row| {
            row.iter().enumerate().fold(
                (row[0], 0),
                |acc, (index, &x)| {
                    if x < acc.0 {
                        (x, index)
                    } else {
                        acc
                    }
                },
            )
        })
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap();
    let lucky = matrix.iter().all(|row| row[max_index] <= max_in_row);
    if lucky {
        vec![max_in_row]
    } else {
        vec![]
    }
}

#[test]
pub fn t1() {
    assert_eq!(
        lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
        vec![15]
    );
    assert_eq!(
        lucky_numbers(vec![vec![3, 6], vec![7, 1], vec![5, 2], vec![4, 8]]),
        vec![]
    )
}
