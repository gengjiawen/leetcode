// https://leetcode.com/problems/image-smoother
//
// An **image smoother** is a filter of the size `3 x 3` that can be applied to each cell of an image by rounding down the average of the cell and the eight surrounding cells (i.e., the average of the nine cells in the blue smoother). If one or more of the surrounding cells of a cell is not present, we do not consider it in the average (i.e., the average of the four cells in the red smoother).
//
// ![](https://assets.leetcode.com/uploads/2021/05/03/smoother-grid.jpg)
//
// Given an `m x n` integer matrix `img` representing the grayscale of an image, return _the image after applying the smoother on each cell of it_.
//
// **Example 1:**
//
// ![](https://assets.leetcode.com/uploads/2021/05/03/smooth-grid.jpg)
// ```
// **Input:** img = [[1,1,1],[1,0,1],[1,1,1]]
// **Output:** [[0,0,0],[0,0,0],[0,0,0]]
// **Explanation:**
// For the points (0,0), (0,2), (2,0), (2,2): floor(3/4) = floor(0.75) = 0
// For the points (0,1), (1,0), (1,2), (2,1): floor(5/6) = floor(0.83333333) = 0
// For the point (1,1): floor(8/9) = floor(0.88888889) = 0
// ```
//
// **Example 2:**
//
// ![](https://assets.leetcode.com/uploads/2021/05/03/smooth2-grid.jpg)
// ```
// **Input:** img = [[100,200,100],[200,50,200],[100,200,100]]
// **Output:** [[137,141,137],[141,138,141],[137,141,137]]
// **Explanation:**
// For the points (0,0), (0,2), (2,0), (2,2): floor((100+200+200+50)/4) = floor(137.5) = 137
// For the points (0,1), (1,0), (1,2), (2,1): floor((200+200+50+200+100+100)/6) = floor(141.666667) = 141
// For the point (1,1): floor((50+200+200+200+200+100+100+100+100)/9) = floor(138.888889) = 138
// ```
//
// **Constraints:**
//
// *   `m == img.length`
// *   `n == img[i].length`
// *   `1 <= m, n <= 200`
// *   `0 <= img[i][j] <= 255`

pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = img.len();
    let n = img[0].len();
    let mut res = vec![vec![0; n]; m];
    for i in 0..m as i32 {
        for j in 0..n as i32 {
            let mut sum = 0;
            let mut count = 0;
            for x in i - 1..=i + 1 {
                for y in j - 1..=j + 1 {
                    if x >= 0 && x < m as i32 && y >= 0 && y < n as i32{
                        sum += img[x as usize][y as usize];
                        count += 1;
                    }
                }
            }
            res[i as usize][j as usize] = sum / count;
        }
    }
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(
        image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
    );
}
