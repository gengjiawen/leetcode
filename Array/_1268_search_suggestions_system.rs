// https://leetcode.com/problems/search-suggestions-system
//
// You are given an array of strings `products` and a string `searchWord`.
//
// Design a system that suggests at most three product names from `products` after each character of `searchWord` is typed. Suggested products should have common prefix with `searchWord`. If there are more than three products with a common prefix return the three lexicographically minimums products.
//
// Return _a list of lists of the suggested products after each character of_ `searchWord` _is typed_.
//
// **Example 1:**
//
// ```
// **Input:** products = ["mobile","mouse","moneypot","monitor","mousepad"], searchWord = "mouse"
// **Output:** [
// ["mobile","moneypot","monitor"],
// ["mobile","moneypot","monitor"],
// ["mouse","mousepad"],
// ["mouse","mousepad"],
// ["mouse","mousepad"]
// ]
// **Explanation:** products sorted lexicographically = ["mobile","moneypot","monitor","mouse","mousepad"]
// After typing m and mo all products match and we show user ["mobile","moneypot","monitor"]
// After typing mou, mous and mouse the system suggests ["mouse","mousepad"]
// ```
//
// **Example 2:**
//
// ```
// **Input:** products = ["havana"], searchWord = "havana"
// **Output:** [["havana"],["havana"],["havana"],["havana"],["havana"],["havana"]]
// ```
//
// **Example 3:**
//
// ```
// **Input:** products = ["bags","baggage","banner","box","cloths"], searchWord = "bags"
// **Output:** [["baggage","bags","banner"],["baggage","bags","banner"],["baggage","bags"],["bags"]]
// ```
//
// **Constraints:**
//
// *   `1 <= products.length <= 1000`
// *   `1 <= products[i].length <= 3000`
// *   `1 <= sum(products[i].length) <= 2 * 10<sup>4</sup>`
// *   All the strings of `products` are **unique**.
// *   `products[i]` consists of lowercase English letters.
// *   `1 <= searchWord.length <= 1000`
// *   `searchWord` consists of lowercase English letters.

pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    let mut ret = Vec::new();

    products.sort();
    let mut current_str = String::new();
    for char in search_word.chars() {
        current_str.push(char);
        let mut matched: Vec<String> = Vec::new();
        for str in products.iter() {
            if str.starts_with(&current_str) {
                matched.push(str.clone());
            }
            if matched.len() == 3 {
                break;
            }
        }
        ret.push(matched);
    }

    return ret;
}

#[test]
pub fn t1() {
    assert_eq!(
        suggested_products(
            vec!["mobile", "mouse", "moneypot", "monitor", "mousepad"]
                .iter()
                .map(|&x| x.to_string())
                .collect(),
            "mouse".to_string()
        ),
        vec![
            vec!["mobile", "moneypot", "monitor"],
            vec!["mobile", "moneypot", "monitor"],
            vec!["mouse", "mousepad"],
            vec!["mouse", "mousepad"],
            vec!["mouse", "mousepad"],
        ]
    );
}
