// https://leetcode.com/problems/slowest-key
//
// A newly designed keypad was tested, where a tester pressed a sequence of `n` keys, one at a time.
//
// You are given a string `keysPressed` of length `n`, where `keysPressed[i]` was the `i<sup>th</sup>` key pressed in the testing sequence, and a sorted list `releaseTimes`, where `releaseTimes[i]` was the time the `i<sup>th</sup>` key was released. Both arrays are **0-indexed**. The `0<sup>th</sup>` key was pressed at the time `0`, and every subsequent key was pressed at the **exact** time the previous key was released.
//
// The tester wants to know the key of the keypress that had the **longest duration**. The `i<sup>th</sup>`keypress had a **duration** of `releaseTimes[i] - releaseTimes[i - 1]`, and the `0<sup>th</sup>` keypress had a duration of `releaseTimes[0]`.
//
// Note that the same key could have been pressed multiple times during the test, and these multiple presses of the same key **may not** have had the same **duration**.
//
// _Return the key of the keypress that had the **longest duration**. If there are multiple such keypresses, return the lexicographically largest key of the keypresses._
//
// **Example 1:**
//
// ```
// **Input:** releaseTimes = [9,29,49,50], keysPressed = "cbcd"
// **Output:** "c"
// **Explanation:** The keypresses were as follows:
// Keypress for 'c' had a duration of 9 (pressed at time 0 and released at time 9).
// Keypress for 'b' had a duration of 29 - 9 = 20 (pressed at time 9 right after the release of the previous character and released at time 29).
// Keypress for 'c' had a duration of 49 - 29 = 20 (pressed at time 29 right after the release of the previous character and released at time 49).
// Keypress for 'd' had a duration of 50 - 49 = 1 (pressed at time 49 right after the release of the previous character and released at time 50).
// The longest of these was the keypress for 'b' and the second keypress for 'c', both with duration 20.
// 'c' is lexicographically larger than 'b', so the answer is 'c'.
// ```
//
// **Example 2:**
//
// ```
// **Input:** releaseTimes = [12,23,36,46,62], keysPressed = "spuda"
// **Output:** "a"
// **Explanation:** The keypresses were as follows:
// Keypress for 's' had a duration of 12.
// Keypress for 'p' had a duration of 23 - 12 = 11.
// Keypress for 'u' had a duration of 36 - 23 = 13.
// Keypress for 'd' had a duration of 46 - 36 = 10.
// Keypress for 'a' had a duration of 62 - 46 = 16.
// The longest of these was the keypress for 'a' with duration 16.```
//
// **Constraints:**
//
// *   `releaseTimes.length == n`
// *   `keysPressed.length == n`
// *   `2 <= n <= 1000`
// *   `1 <= releaseTimes[i] <= 10<sup>9</sup>`
// *   `releaseTimes[i] < releaseTimes[i+1]`
// *   `keysPressed` contains only lowercase English letters.

pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
    let mut max_time = release_times[0];
    let chars = keys_pressed.chars().collect::<Vec<char>>();
    let mut max_char = chars[0];
    for i in 1..chars.len() {
        let diff = release_times[i] - release_times[i - 1];
        if diff > max_time || (diff == max_time && chars[i] > max_char) {
            max_time = diff;
            max_char = chars[i];
        }
    }
    return max_char;
}

#[test]
pub fn t1() {
    assert_eq!(slowest_key(vec![9, 29, 49, 50], "cbcd".to_string()), 'c');
    assert_eq!(
        slowest_key(vec![12, 23, 36, 46, 62], "spuda".to_string()),
        'a'
    );
}
