// https://leetcode.com/problems/intersection-of-three-sorted-arrays

pub fn arrays_intersection(arr1: Vec<i32>, arr2: Vec<i32>, arr3: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let mut array_set = vec![&arr1, &arr2, &arr3];
    let (min_index, _) = array_set
        .iter()
        .enumerate()
        .min_by_key(|(_, i)| i.len())
        .unwrap();
    let min_array = array_set.remove(min_index);
    for i in min_array {
        if array_set[0].contains(&i) && array_set[1].contains(&i) {
            res.push(*i);
        }
    }
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(
        arrays_intersection(
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 5, 7, 9],
            vec![1, 3, 4, 5, 8]
        ),
        vec![1, 5]
    );
}
