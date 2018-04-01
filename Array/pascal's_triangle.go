// https://leetcode.com/problems/pascals-triangle
// 
// Given _numRows_, generate the first _numRows_ of Pascal's triangle.
// 
// For example, given _numRows_ = 5,
// Return
// 
// ```
// [
//      [1],
//     [1,1],
//    [1,2,1],
//   [1,3,3,1],
//  [1,4,6,4,1]
// ]
// ```
package main
func generate(numRows int) [][]int {
    rets := make([][]int, numRows)
    for i := 0; i < numRows; i++ {
        rets[i] = make([]int, i+1)
        rets[i][0] = 1
        rets[i][i] = 1
        for j := 1; j < i; j++ {
            rets[i][j] = rets[i-1][j-1] + rets[i-1][j]
        }
    }
    return rets
}