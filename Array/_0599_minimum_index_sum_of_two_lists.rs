// https://leetcode.com/problems/minimum-index-sum-of-two-lists
//
// Suppose Andy and Doris want to choose a restaurant for dinner, and they both have a list of favorite restaurants represented by strings.
//
// You need to help them find out their **common interest** with the **least list index sum**. If there is a choice tie between answers, output all of them with no order requirement. You could assume there always exists an answer.
//
// **Example 1:**
//
// ```
// **Input:** list1 = ["Shogun","Tapioca Express","Burger King","KFC"], list2 = ["Piatti","The Grill at Torrey Pines","Hungry Hunter Steakhouse","Shogun"]
// **Output:** ["Shogun"]
// **Explanation:** The only restaurant they both like is "Shogun".
// ```
//
// **Example 2:**
//
// ```
// **Input:** list1 = ["Shogun","Tapioca Express","Burger King","KFC"], list2 = ["KFC","Shogun","Burger King"]
// **Output:** ["Shogun"]
// **Explanation:** The restaurant they both like and have the least index sum is "Shogun" with index sum 1 (0+1).
// ```
//
// **Constraints:**
//
// *   `1 <= list1.length, list2.length <= 1000`
// *   `1 <= list1[i].length, list2[i].length <= 30`
// *   `list1[i]` and `list2[i]` consist of spaces `' '` and English letters.
// *   All the stings of `list1` are **unique**.
// *   All the stings of `list2` are **unique**.

pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let list1_map = list1
        .iter()
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect::<std::collections::HashMap<_, _>>();
    let mut res = vec![];
    let mut min_sum = std::i32::MAX as usize;
    for i in 0..list2.len() {
        if let Some(j) = list1_map.get(&list2[i]) {
            if i + j == min_sum {
                res.push(list1[*j].clone());
            }
            if i + j < min_sum {
                min_sum = i + j;
                res.clear();
                res.push(list1[*j].clone());
            }
        }
    }
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(
        find_restaurant(
            vec![
                "Shogun".to_string(),
                "Tapioca Express".to_string(),
                "Burger King".to_string(),
                "KFC".to_string()
            ],
            vec![
                "Piatti".to_string(),
                "The Grill at Torrey Pines".to_string(),
                "Hungry Hunter Steakhouse".to_string(),
                "Shogun".to_string()
            ]
        ),
        vec!["Shogun".to_string()]
    );
}
