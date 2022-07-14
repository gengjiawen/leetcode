// https://leetcode.com/problems/build-an-array-with-stack-operations
//
// You are given an array `target` and an integer `n`.
//
// In each iteration, you will read a number from `list = [1, 2, 3, ..., n]`.
//
// Build the `target` array using the following operations:
//
// *   `"Push"`: Reads a new element from the beginning list, and pushes it in the array.
// *   `"Pop"`: Deletes the last element of the array.
// *   If the target array is already built, stop reading more elements.
//
// Return _a list of the operations needed to build_ `target`. The test cases are generated so that the answer is **unique**.
//
// **Example 1:**
//
// ```
// **Input:** target = [1,3], n = 3
// **Output:** ["Push","Push","Pop","Push"]
// **Explanation:**
// Read number 1 and automatically push in the array -> [1]
// Read number 2 and automatically push in the array then Pop it -> [1]
// Read number 3 and automatically push in the array -> [1,3]
// ```
//
// **Example 2:**
//
// ```
// **Input:** target = [1,2,3], n = 3
// **Output:** ["Push","Push","Push"]
// ```
//
// **Example 3:**
//
// ```
// **Input:** target = [1,2], n = 4
// **Output:** ["Push","Push"]
// **Explanation:** You only need to read the first 2 numbers and stop.
// ```
//
// **Constraints:**
//
// *   `1 <= target.length <= 100`
// *   `1 <= n <= 100`
// *   `1 <= target[i] <= n`
// *   `target` is strictly increasing.

pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
    let mut res = Vec::new();
    let mut index = 0;
    for i in 1..=target[target.len() - 1] {
        if i == target[index] {
            res.push("Push".to_string());
            index += 1;
        } else {
            res.push("Push".to_string());
            res.push("Pop".to_string());
        }
    }
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(
        build_array(vec![1, 3], 3),
        vec![
            String::from("Push"),
            String::from("Push"),
            String::from("Pop"),
            String::from("Push")
        ]
    );
    assert_eq!(
        build_array(vec![1, 2, 3], 3),
        vec![
            String::from("Push"),
            String::from("Push"),
            String::from("Push")
        ]
    );
}
