// https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards
//
// There are several cards **arranged in a row**, and each card has an associated number of points.
// The points are given in the integer array `cardPoints`.
//
// In one step, you can take one card from the beginning or from the end of the row. You have to take exactly `k` cards.
//
// Your score is the sum of the points of the cards you have taken.
//
// Given the integer array `cardPoints` and the integer `k`, return the _maximum score_ you can obtain.
//
// **Example 1:**
//
// ```
// **Input:** cardPoints = [1,2,3,4,5,6,1], k = 3
// **Output:** 12
// **Explanation:** After the first step, your score will always be 1\. However, choosing the rightmost card first will maximize your total score. The optimal strategy is to take the three cards on the right, giving a final score of 1 + 6 + 5 = 12.
// ```
//
// **Example 2:**
//
// ```
// **Input:** cardPoints = [2,2,2], k = 2
// **Output:** 4
// **Explanation:** Regardless of which two cards you take, your score will always be 4.
// ```
//
// **Example 3:**
//
// ```
// **Input:** cardPoints = [9,7,7,9,7,7,9], k = 7
// **Output:** 55
// **Explanation:** You have to take all the cards. Your score is the sum of points of all cards.
// ```
//
// **Constraints:**
//
// *   `1 <= cardPoints.length <= 10<sup>5</sup>`
// *   `1 <= cardPoints[i] <= 10<sup>4</sup>`
// *   `1 <= k <= cardPoints.length`

pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let all_sum = card_points.iter().sum::<i32>();
    let window_len = (card_points.len() - k as usize) as i32;
    let mut curr_sum = card_points[0..window_len as usize].iter().sum::<i32>();
    let mut min_sum = curr_sum;
    for end in window_len as usize..card_points.len() {
        curr_sum = curr_sum + card_points[end] - card_points[end - window_len as usize];
        min_sum = min_sum.min(curr_sum);
    }

    return all_sum - min_sum;
}

#[test]
pub fn test() {
    assert_eq!(max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
}

