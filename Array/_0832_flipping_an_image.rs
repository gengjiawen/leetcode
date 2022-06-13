// https://leetcode.com/problems/flipping-an-image
//
// Given an `n x n` binary matrix `image`, flip the image **horizontally**, then invert it, and return _the resulting image_.
//
// To flip an image horizontally means that each row of the image is reversed.
//
// *   For example, flipping `[1,1,0]` horizontally results in `[0,1,1]`.
//
// To invert an image means that each `0` is replaced by `1`, and each `1` is replaced by `0`.
//
// *   For example, inverting `[0,1,1]` results in `[1,0,0]`.
//
// **Example 1:**
//
// ```
// **Input:** image = [[1,1,0],[1,0,1],[0,0,0]]
// **Output:** [[1,0,0],[0,1,0],[1,1,1]]
// **Explanation:** First reverse each row: [[0,1,1],[1,0,1],[0,0,0]].
// Then, invert the image: [[1,0,0],[0,1,0],[1,1,1]]
// ```
//
// **Example 2:**
//
// ```
// **Input:** image = [[1,1,0,0],[1,0,0,1],[0,1,1,1],[1,0,1,0]]
// **Output:** [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
// **Explanation:** First reverse each row: [[0,0,1,1],[1,0,0,1],[1,1,1,0],[0,1,0,1]].
// Then invert the image: [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
// ```
//
// **Constraints:**
//
// *   `n == image.length`
// *   `n == image[i].length`
// *   `1 <= n <= 20`
// *   `images[i][j]` is either `0` or `1`.

pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    return image
        .into_iter()
        .map(|row| row.iter().rev().map(|x| x ^ 1).collect())
        .collect();
}

#[test]
pub fn t1() {
    assert_eq!(
        flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
        vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
    );
}
