// https://leetcode.com/problems/counting-elements

pub fn count_elements(arr: Vec<i32>) -> i32 {
    let mut map = std::collections::HashMap::new();
    for i in &arr {
        *map.entry(i).or_insert(0) += 1;
    }

    let mut count = 0;
    for i in &arr {
        if map.contains_key(&(*i + 1)) {
            count += 1;
        }
    }
    return count;
}

#[test]
pub fn t1() {
    assert_eq!(count_elements(vec![1, 2, 3]), 2);
}
