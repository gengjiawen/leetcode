// https://leetcode.com/problems/fair-candy-swap
//
// Alice and Bob have a different total number of candies. You are given two integer arrays `aliceSizes` and `bobSizes` where `aliceSizes[i]` is the number of candies of the `i<sup>th</sup>` box of candy that Alice has and `bobSizes[j]` is the number of candies of the `j<sup>th</sup>` box of candy that Bob has.
//
// Since they are friends, they would like to exchange one candy box each so that after the exchange, they both have the same total amount of candy. The total amount of candy a person has is the sum of the number of candies in each box they have.
//
// Return a_n integer array_ `answer` _where_ `answer[0]` _is the number of candies in the box that Alice must exchange, and_ `answer[1]` _is the number of candies in the box that Bob must exchange_. If there are multiple answers, you may **return any** one of them. It is guaranteed that at least one answer exists.
//
// **Example 1:**
//
// ```
// **Input:** aliceSizes = [1,1], bobSizes = [2,2]
// **Output:** [1,2]
// ```
//
// **Example 2:**
//
// ```
// **Input:** aliceSizes = [1,2], bobSizes = [2,3]
// **Output:** [1,2]
// ```
//
// **Example 3:**
//
// ```
// **Input:** aliceSizes = [2], bobSizes = [1,3]
// **Output:** [2,3]
// ```
//
// **Constraints:**
//
// *   `1 <= aliceSizes.length, bobSizes.length <= 10<sup>4</sup>`
// *   `1 <= aliceSizes[i], bobSizes[j] <= 10<sup>5</sup>`
// *   Alice and Bob have a different total number of candies.
// *   There will be at least one valid answer for the given input.

pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
    let alice_sum = alice_sizes.iter().sum::<i32>();
    let bob_sum = bob_sizes.iter().sum::<i32>();
    for i in 0..alice_sizes.len() {
        let alice_size = alice_sizes[i];
        for j in 0..bob_sizes.len() {
            let bob_size = bob_sizes[j];
            if alice_sum + bob_size - alice_size == bob_sum + alice_size - bob_size {
                return vec![alice_size, bob_size];
            }
        }
    }
    return vec![0, 0];
}

#[test]
pub fn t1() {
    assert_eq!(fair_candy_swap(vec![1, 1], vec![2, 2]), vec![1, 2]);
    assert_eq!(fair_candy_swap(vec![1, 2], vec![2, 3]), vec![1, 2]);
    assert_eq!(fair_candy_swap(vec![2], vec![1, 3]), vec![2, 3]);
}
