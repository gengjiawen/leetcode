// https://leetcode.com/problems/merge-similar-items
//
// You are given two 2D integer arrays, `items1` and `items2`, representing two sets of items. Each array `items` has the following properties:
//
// *   `items[i] = [value<sub>i</sub>, weight<sub>i</sub>]` where `value<sub>i</sub>` represents the **value** and `weight<sub>i</sub>` represents the **weight** of the `i<sup>th</sup>` item.
// *   The value of each item in `items` is **unique**.
//
// Return _a 2D integer array_ `ret` _where_ `ret[i] = [value<sub>i</sub>, weight<sub>i</sub>]`_,_ _with_ `weight<sub>i</sub>` _being the **sum of weights** of all items with value_ `value<sub>i</sub>`.
//
// **Note:** `ret` should be returned in **ascending** order by value.
//
// **Example 1:**
//
// ```
// **Input:** items1 = [[1,1],[4,5],[3,8]], items2 = [[3,1],[1,5]]
// **Output:** [[1,6],[3,9],[4,5]]
// **Explanation:**
// The item with value = 1 occurs in items1 with weight = 1 and in items2 with weight = 5, total weight = 1 + 5 = 6.
// The item with value = 3 occurs in items1 with weight = 8 and in items2 with weight = 1, total weight = 8 + 1 = 9.
// The item with value = 4 occurs in items1 with weight = 5, total weight = 5\.
// Therefore, we return [[1,6],[3,9],[4,5]].
// ```
//
// **Example 2:**
//
// ```
// **Input:** items1 = [[1,1],[3,2],[2,3]], items2 = [[2,1],[3,2],[1,3]]
// **Output:** [[1,4],[2,4],[3,4]]
// **Explanation:**
// The item with value = 1 occurs in items1 with weight = 1 and in items2 with weight = 3, total weight = 1 + 3 = 4.
// The item with value = 2 occurs in items1 with weight = 3 and in items2 with weight = 1, total weight = 3 + 1 = 4.
// The item with value = 3 occurs in items1 with weight = 2 and in items2 with weight = 2, total weight = 2 + 2 = 4.
// Therefore, we return [[1,4],[2,4],[3,4]].```
//
// **Example 3:**
//
// ```
// **Input:** items1 = [[1,3],[2,2]], items2 = [[7,1],[2,2],[1,4]]
// **Output:** [[1,7],[2,4],[7,1]]
// **Explanation:** The item with value = 1 occurs in items1 with weight = 3 and in items2 with weight = 4, total weight = 3 + 4 = 7\.
// The item with value = 2 occurs in items1 with weight = 2 and in items2 with weight = 2, total weight = 2 + 2 = 4\.
// The item with value = 7 occurs in items2 with weight = 1, total weight = 1.
// Therefore, we return [[1,7],[2,4],[7,1]].
// ```
//
// **Constraints:**
//
// *   `1 <= items1.length, items2.length <= 1000`
// *   `items1[i].length == items2[i].length == 2`
// *   `1 <= value<sub>i</sub>, weight<sub>i</sub> <= 1000`
// *   Each `value<sub>i</sub>` in `items1` is **unique**.
// *   Each `value<sub>i</sub>` in `items2` is **unique**.

pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result_hash = std::collections::HashMap::new();
    for item in items1 {
        *result_hash.entry(item[0]).or_default() += item[1]
    }
    for item in items2 {
        *result_hash.entry(item[0]).or_default() += item[1]
    }
    let mut res = vec![];
    for (key, value) in result_hash {
        res.push(vec![key, value]);
    }
    res.sort_by(|a, b| a[0].cmp(&b[0]));
    return res;
}

#[test]
pub fn t1() {
    let items1 = vec![vec![1, 1], vec![4, 5], vec![3, 8]];
    let items2 = vec![vec![3, 1], vec![1, 5]];
    let ret = vec![vec![1, 6], vec![3, 9], vec![4, 5]];
    assert_eq!(merge_similar_items(items1, items2), ret);
}
