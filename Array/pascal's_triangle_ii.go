// https://leetcode.com/problems/pascals-triangle-ii
//
// Given an index _k_, return the _k_<sup>th</sup> row of the Pascal's triangle.
//
// For example, given _k_ = 3,
// Return `[1,3,3,1]`.
//
// **Note:**
// Could you optimize your algorithm to use only _O_(_k_) extra space?

package main

import "fmt"

func getRow(rowIndex int) []int {
	rets := make([]int, rowIndex + 1)
	rets[0] = 1
	for i := 1; i <= rowIndex; i++ {
        rets[i] = 1
		for j := i - 1; j > 0; j-- {
			rets[j] = rets[j] + rets[j-1]
		}
        //fmt.Printf("middle %v\n", rets)
	}
	return rets
}

func main() {
    s := getRow(3)
    fmt.Printf("%v", s)
}
