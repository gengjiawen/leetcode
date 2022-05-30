// https://leetcode.com/problems/find-anagram-mappings

pub fn anagram_mappings(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    nums2.iter().enumerate().for_each(|(i, &v)| {
        map.insert(v, i as i32);
    });
    let mut res = vec![];
    nums1.iter().for_each(|&v| {
        res.push(map[&v]);
    });
    return res;
}

#[test]
pub fn test() {
    assert_eq!(anagram_mappings(vec![84, 46], vec![84, 46]), vec![0, 1]);
}
