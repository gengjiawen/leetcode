// https://leetcode.com/problems/contains-duplicate-ii
// 
// Given an array of integers and an integer _k_, find out whether there are two distinct indices _i_ and _j_ in the array such that **nums[i] = nums[j]** and the **absolute** difference between _i_ and _j_ is at most _k_.
package main


func containsNearbyDuplicate(nums []int, k int) bool {
    if len(nums) < 2 {
        return false
    }
    m := make(map[int]int)
    for i := range nums {
        if j, ok := m[nums[i]]; ok {
            if i - j <= k {
                return true
            }
        }
        m[nums[i]] = i
    }
    return false
}

