// https://leetcode.com/problems/count-items-matching-a-rule
//
// You are given an array `items`, where each `items[i] = [type<sub>i</sub>, color<sub>i</sub>, name<sub>i</sub>]` describes the type, color, and name of the `i<sup>th</sup>` item. You are also given a rule represented by two strings, `ruleKey` and `ruleValue`.
//
// The `i<sup>th</sup>` item is said to match the rule if **one** of the following is true:
//
// *   `ruleKey == "type"` and `ruleValue == type<sub>i</sub>`.
// *   `ruleKey == "color"` and `ruleValue == color<sub>i</sub>`.
// *   `ruleKey == "name"` and `ruleValue == name<sub>i</sub>`.
//
// Return _the number of items that match the given rule_.
//
// **Example 1:**
//
// ```
// **Input:** items = [["phone","blue","pixel"],["computer","silver","lenovo"],["phone","gold","iphone"]], ruleKey = "color", ruleValue = "silver"
// **Output:** 1
// **Explanation:** There is only one item matching the given rule, which is ["computer","silver","lenovo"].
// ```
//
// **Example 2:**
//
// ```
// **Input:** items = [["phone","blue","pixel"],["computer","silver","phone"],["phone","gold","iphone"]], ruleKey = "type", ruleValue = "phone"
// **Output:** 2
// **Explanation:** There are only two items matching the given rule, which are ["phone","blue","pixel"] and ["phone","gold","iphone"]. Note that the item ["computer","silver","phone"] does not match.```
//
// **Constraints:**
//
// *   `1 <= items.length <= 10<sup>4</sup>`
// *   `1 <= type<sub>i</sub>.length, color<sub>i</sub>.length, name<sub>i</sub>.length, ruleValue.length <= 10`
// *   `ruleKey` is equal to either `"type"`, `"color"`, or `"name"`.
// *   All strings consist only of lowercase letters.

pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    let mut count = 0;
    let mut index = 0;
    if rule_key == "color" {
        index = 1;
    } else if rule_key == "name" {
        index = 2;
    }

    for item in items {
        if item[index] == rule_value {
            count += 1;
        }
    }
    return count;
}

#[test]
pub fn t1() {
    assert_eq!(
        count_matches(
            vec![
                vec!["phone", "blue", "pixel"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
                vec!["computer", "silver", "lenovo"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
                vec!["phone", "gold", "iphone"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
            ],
            "color".to_string(),
            "silver".to_string()
        ),
        1
    );
}
