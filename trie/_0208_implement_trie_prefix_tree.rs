// https://leetcode.com/problems/implement-trie-prefix-tree
//
// A [**trie**](https://en.wikipedia.org/wiki/Trie) (pronounced as "try") or **prefix tree** is a tree data structure used to efficiently store and retrieve keys in a dataset of strings. There are various applications of this data structure, such as autocomplete and spellchecker.
//
// Implement the Trie class:
//
// *   `Trie()` Initializes the trie object.
// *   `void insert(String word)` Inserts the string `word` into the trie.
// *   `boolean search(String word)` Returns `true` if the string `word` is in the trie (i.e., was inserted before), and `false` otherwise.
// *   `boolean startsWith(String prefix)` Returns `true` if there is a previously inserted string `word` that has the prefix `prefix`, and `false` otherwise.
//
// **Example 1:**
//
// ```
// **Input**
// ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
// [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
// **Output**
// [null, null, true, false, true, null, true]
//
// **Explanation**
// Trie trie = new Trie();
// trie.insert("apple");
// trie.search("apple");   // return True
// trie.search("app");     // return False
// trie.startsWith("app"); // return True
// trie.insert("app");
// trie.search("app");     // return True
// ```
//
// **Constraints:**
//
// *   `1 <= word.length, prefix.length <= 2000`
// *   `word` and `prefix` consist only of lowercase English letters.
// *   At most `3 * 10<sup>4</sup>` calls **in total** will be made to `insert`, `search`, and `startsWith`.

use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            end: false,
        }
    }

    fn insert(&mut self, word: String) {
        let mut trie = self;
        for c in word.chars() {
            trie = trie.children.entry(c).or_insert(Trie::new());
        }
        trie.end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut trie = self;
        for c in word.chars() {
            if let Some(next) = trie.children.get(&c) {
                trie = next;
            } else {
                return false;
            }
        }
        return trie.end;
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut trie = self;
        for c in prefix.chars() {
            if let Some(t) = trie.children.get(&c) {
                trie = t;
            } else {
                return false;
            }
        }
        return true;
    }
}

#[test]
pub fn t1() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert_eq!(trie.search("apple".to_string()), true);
    assert_eq!(trie.search("app".to_string()), false);
    assert_eq!(trie.starts_with("app".to_string()), true);
    trie.insert("app".to_string());
    assert_eq!(trie.search("app".to_string()), true);
}
