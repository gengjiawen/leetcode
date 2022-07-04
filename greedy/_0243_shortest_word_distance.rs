// https://leetcode.com/problems/shortest-word-distance

pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
    let mut min_dist = std::i32::MAX;
    for i in 0..words_dict.len() {
        if words_dict[i] == word1 {
            for j in 0..words_dict.len() {
                if words_dict[j] == word2 {
                    min_dist = min_dist.min((i as i32 - j as i32).abs());
                    if min_dist == 1 {
                        return 1;
                    }
                }
            }
        }
    }
    return min_dist;
}

#[test]
pub fn t1() {
    assert_eq!(
        shortest_distance(
            vec![
                "practice".to_string(),
                "makes".to_string(),
                "perfect".to_string(),
                "coding".to_string(),
                "makes".to_string()
            ],
            "coding".to_string(),
            "practice".to_string()
        ),
        3
    );
}
