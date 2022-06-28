// https://leetcode.com/problems/island-perimeter
//
// You are given `row x col` `grid` representing a map where `grid[i][j] = 1` represents land and `grid[i][j] = 0` represents water.
//
// Grid cells are connected **horizontally/vertically** (not diagonally). The `grid` is completely surrounded by water, and there is exactly one island (i.e., one or more connected land cells).
//
// The island doesn't have "lakes", meaning the water inside isn't connected to the water around the island. One cell is a square with side length 1\. The grid is rectangular, width and height don't exceed 100\. Determine the perimeter of the island.
//
// **Example 1:**
//
// ![](https://assets.leetcode.com/uploads/2018/10/12/island.png)
// ```
// **Input:** grid = [[0,1,0,0],[1,1,1,0],[0,1,0,0],[1,1,0,0]]
// **Output:** 16
// **Explanation:** The perimeter is the 16 yellow stripes in the image above.
// ```
//
// **Example 2:**
//
// ```
// **Input:** grid = [[1]]
// **Output:** 4
// ```
//
// **Example 3:**
//
// ```
// **Input:** grid = [[1,0]]
// **Output:** 4
// ```
//
// **Constraints:**
//
// *   `row == grid.length`
// *   `col == grid[i].length`
// *   `1 <= row, col <= 100`
// *   `grid[i][j]` is `0` or `1`.
// *   There is exactly one island in `grid`.

pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let mut perimeter = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                perimeter += 4;
                if i > 0 && grid[i - 1][j] == 1 {
                    perimeter -= 2;
                }
                if j > 0 && grid[i][j - 1] == 1 {
                    perimeter -= 2;
                }
            }
        }
    }
    return perimeter;
}

#[test]
pub fn t1() {
    assert_eq!(
        island_perimeter(vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0]
        ]),
        16
    );
}
