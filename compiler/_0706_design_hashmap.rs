// https://leetcode.com/problems/design-hashmap
//
// Design a HashMap without using any built-in hash table libraries.
//
// Implement the `MyHashMap` class:
//
// *   `MyHashMap()` initializes the object with an empty map.
// *   `void put(int key, int value)` inserts a `(key, value)` pair into the HashMap. If the `key` already exists in the map, update the corresponding `value`.
// *   `int get(int key)` returns the `value` to which the specified `key` is mapped, or `-1` if this map contains no mapping for the `key`.
// *   `void remove(key)` removes the `key` and its corresponding `value` if the map contains the mapping for the `key`.
//
// **Example 1:**
//
// ```
// **Input**
// ["MyHashMap", "put", "put", "get", "get", "put", "get", "remove", "get"]
// [[], [1, 1], [2, 2], [1], [3], [2, 1], [2], [2], [2]]
// **Output**
// [null, null, null, 1, -1, null, 1, null, -1]
//
// **Explanation**
// MyHashMap myHashMap = new MyHashMap();
// myHashMap.put(1, 1); // The map is now [[1,1]]
// myHashMap.put(2, 2); // The map is now [[1,1], [2,2]]
// myHashMap.get(1);    // return 1, The map is now [[1,1], [2,2]]
// myHashMap.get(3);    // return -1 (i.e., not found), The map is now [[1,1], [2,2]]
// myHashMap.put(2, 1); // The map is now [[1,1], [2,1]] (i.e., update the existing value)
// myHashMap.get(2);    // return 1, The map is now [[1,1], [2,1]]
// myHashMap.remove(2); // remove the mapping for 2, The map is now [[1,1]]
// myHashMap.get(2);    // return -1 (i.e., not found), The map is now [[1,1]]
// ```
//
// **Constraints:**
//
// *   `0 <= key, value <= 10<sup>6</sup>`
// *   At most `10<sup>4</sup>` calls will be made to `put`, `get`, and `remove`.

struct MyHashMap {
    table: Vec<i32>,
}

impl MyHashMap {
    fn new() -> Self {
        MyHashMap {
            table: vec![-1; 1_000_001],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.table[key as usize] = value;
    }

    fn get(&self, key: i32) -> i32 {
        return self.table[key as usize];
    }

    fn remove(&mut self, key: i32) {
        self.table[key as usize] = -1;
    }
}

#[test]
pub fn t1() {
    let mut my_hash_map = MyHashMap::new();
    my_hash_map.put(1, 1);
    my_hash_map.put(2, 2);
    assert_eq!(my_hash_map.get(1), 1);
    assert_eq!(my_hash_map.get(3), -1);
    my_hash_map.put(2, 1);
    assert_eq!(my_hash_map.get(2), 1);
    my_hash_map.remove(2);
    assert_eq!(my_hash_map.get(2), -1);
}
