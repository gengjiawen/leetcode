// https://leetcode.com/problems/design-add-and-search-words-data-structure
//
// Design a data structure that supports adding new words and finding if a string matches any previously added string.
//
// Implement the `WordDictionary` class:
//
// *   `WordDictionary()` Initializes the object.
// *   `void addWord(word)` Adds `word` to the data structure, it can be matched later.
// *   `bool search(word)` Returns `true` if there is any string in the data structure that matches `word` or `false` otherwise. `word` may contain dots `'.'` where dots can be matched with any letter.
//
// **Example:**
//
// ```
// **Input**
// ["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
// [[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
// **Output**
// [null,null,null,null,false,true,true,true]
//
// **Explanation**
// WordDictionary wordDictionary = new WordDictionary();
// wordDictionary.addWord("bad");
// wordDictionary.addWord("dad");
// wordDictionary.addWord("mad");
// wordDictionary.search("pad"); // return False
// wordDictionary.search("bad"); // return True
// wordDictionary.search(".ad"); // return True
// wordDictionary.search("b.."); // return True
// ```
//
// **Constraints:**
//
// *   `1 <= word.length <= 25`
// *   `word` in `addWord` consists of lowercase English letters.
// *   `word` in `search` consist of `'.'` or lowercase English letters.
// *   There will be at most `3` dots in `word` for `search` queries.
// *   At most `10<sup>4</sup>` calls will be made to `addWord` and `search`.

use std::collections::HashMap;

struct WordDictionary {
    children: HashMap<char, WordDictionary>,
    end: bool,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            children: HashMap::new(),
            end: false,
        }
    }

    fn add_word(&mut self, word: String) {
        let mut trie = self;
        for c in word.chars() {
            trie = trie.children.entry(c).or_insert(WordDictionary::new());
        }
        trie.end = true;
    }

    fn search(&self, word: String) -> bool {
        if word.is_empty() {
            return self.end;
        }
        let c = word.chars().next().unwrap();
        if c == '.' {
            for child in self.children.values() {
                if Self::search(child, word[1..].to_string()) {
                    return true;
                }
            }
        } else {
            if let Some(child) = self.children.get(&c) {
                return Self::search(child, word[1..].to_string());
            } else {
                return false;
            }
        }
        return false;
    }
}

#[test]
pub fn t1() {
    let mut trie = WordDictionary::new();
    trie.add_word("bad".to_string());
    trie.add_word("dad".to_string());
    trie.add_word("mad".to_string());
    assert_eq!(trie.search("pad".to_string()), false);
    assert_eq!(trie.search("bad".to_string()), true);
    assert_eq!(trie.search(".ad".to_string()), true);
    assert_eq!(trie.search("b..".to_string()), true);
}
