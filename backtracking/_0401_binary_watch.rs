// https://leetcode.com/problems/binary-watch
//
// A binary watch has 4 LEDs on the top to represent the hours (0-11), and 6 LEDs on the bottom to represent the minutes (0-59). Each LED represents a zero or one, with the least significant bit on the right.
//
// *   For example, the below binary watch reads `"4:51"`.
//
// ![](https://assets.leetcode.com/uploads/2021/04/08/binarywatch.jpg)
//
// Given an integer `turnedOn` which represents the number of LEDs that are currently on (ignoring the PM), return _all possible times the watch could represent_. You may return the answer in **any order**.
//
// The hour must not contain a leading zero.
//
// *   For example, `"01:00"` is not valid. It should be `"1:00"`.
//
// The minute must be consist of two digits and may contain a leading zero.
//
// *   For example, `"10:2"` is not valid. It should be `"10:02"`.
//
// **Example 1:**
//
// ```
// **Input:** turnedOn = 1
// **Output:** ["0:01","0:02","0:04","0:08","0:16","0:32","1:00","2:00","4:00","8:00"]
// ```
//
// **Example 2:**
//
// ```
// **Input:** turnedOn = 9
// **Output:** []
// ```
//
// **Constraints:**
//
// *   `0 <= turnedOn <= 10`

pub fn read_binary_watch_no_backtrack(turned_on: i32) -> Vec<String> {
    let mut ret = Vec::<String>::new();
    for h in 0..12i32 {
        let bit_h = h.count_ones();
        for m in 0..60i32 {
            let bit_m = m.count_ones();
            if bit_h + bit_m == turned_on as u32 {
                ret.push(format!("{}:{:02}", h, m));
            }
        }
    }
    ret
}

pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
    let mut nums = vec![0; 10];
    fn backtrack(cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, turned_on: i32) {
        if turned_on == 0 {
            res.push(cur.clone());
            return;
        }
        for i in 0..10 {
            cur[i] = 1;
            backtrack(cur, res, turned_on - 1);
            cur[i] = 0;
        }
    }
    let mut res = vec![];
    backtrack(&mut nums, &mut res, turned_on);
    let hour_clock = [8, 4, 2, 1];
    let minute_clock = [32, 16, 8, 4, 2, 1];
    return res
        .iter()
        .map(|v| {
            let mut hour = 0;
            let mut minute = 0;
            for i in 0..4 {
                if v[i] == 1 {
                    hour += hour_clock[i];
                }
            }
            for i in 4..10 {
                if v[i] == 1 {
                    minute += minute_clock[i - 4];
                }
            }
            let s = format!("{}:{:02}", hour, minute);
            return s;
        })
        .collect();
}

#[test]
pub fn t1() {
    use std::collections::HashSet;
    let s1: HashSet<String> = HashSet::from_iter(read_binary_watch_no_backtrack(1).iter().cloned());
    let s2: HashSet<String> = HashSet::from_iter(read_binary_watch(1).iter().cloned());
    let result = HashSet::from_iter(
        [
            "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
        ]
        .iter()
        .map(|s| s.to_string()),
    );
    assert_eq!(s1, result);
    assert_eq!(s2, result);
}
