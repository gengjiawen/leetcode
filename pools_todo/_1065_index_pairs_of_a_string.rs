// https://leetcode.com/problems/index-pairs-of-a-string

pub fn index_pairs(text: String, words: Vec<String>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let text_len = text.len();
    let hs: std::collections::HashSet<String> = std::collections::HashSet::from_iter(words);
    for i in 0..text_len {
        for j in i..text_len {
            if hs.contains(&text[i..=j].to_string()) {
                result.push(vec![i as i32, j as i32]);
            }
        }
    }
    return result;
}

#[test]
pub fn t1() {
    assert_eq!(
        index_pairs(
            "thestoryofleetcodeandme".to_string(),
            vec![
                "story".to_string(),
                "fleet".to_string(),
                "leetcode".to_string(),
            ]
        ),
        vec![vec![3, 7], vec![9, 13], vec![10, 17]]
    );
    assert_eq!(
        index_pairs(
            "ababa".to_string(),
            vec![
                "aba".to_string(),
                "ab".to_string(),
            ]
        ),
        vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]]
    );
}

