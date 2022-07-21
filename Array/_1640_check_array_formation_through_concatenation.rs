// https://leetcode.com/problems/check-array-formation-through-concatenation
//
// You are given an array of **distinct** integers `arr` and an array of integer arrays `pieces`, where the integers in `pieces` are **distinct**. Your goal is to form `arr` by concatenating the arrays in `pieces` **in any order**. However, you are **not** allowed to reorder the integers in each array `pieces[i]`.
//
// Return `true` _if it is possible_ _to form the array_ `arr` _from_ `pieces`. Otherwise, return `false`.
//
// **Example 1:**
//
// ```
// **Input:** arr = [15,88], pieces = [[88],[15]]
// **Output:** true
// **Explanation:** Concatenate [15] then [88]
// ```
//
// **Example 2:**
//
// ```
// **Input:** arr = [49,18,16], pieces = [[16,18,49]]
// **Output:** false
// **Explanation:** Even though the numbers match, we cannot reorder pieces[0].
// ```
//
// **Example 3:**
//
// ```
// **Input:** arr = [91,4,64,78], pieces = [[78],[4,64],[91]]
// **Output:** true
// **Explanation:** Concatenate [91] then [4,64] then [78]
// ```
//
// **Constraints:**
//
// *   `1 <= pieces.length <= arr.length <= 100`
// *   `sum(pieces[i].length) == arr.length`
// *   `1 <= pieces[i].length <= arr.length`
// *   `1 <= arr[i], pieces[i][j] <= 100`
// *   The integers in `arr` are **distinct**.
// *   The integers in `pieces` are **distinct** (i.e., If we flatten pieces in a 1D array, all the integers in this array are distinct).

pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
    for piece in pieces {
        let mut piece_index = 0;
        let arr_index_option = arr.iter().enumerate().find(|&x| x.1 == &piece[0]);

        if arr_index_option.is_none() {
            return false;
        }
        let mut arr_index = arr_index_option.unwrap().0;

        while piece_index < piece.len() {
            if arr_index > arr.len() - 1 || piece[piece_index] != arr[arr_index as usize] {
                return false;
            }
            arr_index += 1;
            piece_index += 1;
        }
    }
    return true;
}

#[test]
pub fn t1() {
    assert_eq!(can_form_array(vec![15, 88], vec![vec![88], vec![15]]), true);
    assert_eq!(
        can_form_array(vec![49, 18, 16], vec![vec![16, 18, 19]]),
        false
    );
}
