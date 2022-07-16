// https://leetcode.com/problems/perform-string-shifts

pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
    let mut move_steps = shift.iter().fold(0, |mut acc, x| {
        if x[0] == 0 {
            acc = acc - x[1];
        } else {
            acc = acc + x[1];
        }
        return acc;
    });

    move_steps = move_steps % s.len() as i32;
    if move_steps < 0 {
        move_steps = move_steps + s.len() as i32;
    }
    let mut res = String::new();
    res.push_str(&s[(s.len() - move_steps as usize)..]);
    res.push_str(&s[0..(s.len() - move_steps as usize)]);
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(
        string_shift("abc".to_string(), vec![vec![0, 1], vec![1, 2]]),
        "cab".to_string()
    );
}
