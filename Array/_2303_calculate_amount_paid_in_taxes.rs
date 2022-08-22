// https://leetcode.com/problems/calculate-amount-paid-in-taxes
//
// You are given a **0-indexed** 2D integer array `brackets` where `brackets[i] = [upper<sub>i</sub>, percent<sub>i</sub>]` means that the `i<sup>th</sup>` tax bracket has an upper bound of `upper<sub>i</sub>` and is taxed at a rate of `percent<sub>i</sub>`. The brackets are **sorted** by upper bound (i.e. `upper<sub>i-1</sub> < upper<sub>i</sub>` for `0 < i < brackets.length`).
//
// Tax is calculated as follows:
//
// *   The first `upper<sub>0</sub>` dollars earned are taxed at a rate of `percent<sub>0</sub>`.
// *   The next `upper<sub>1</sub> - upper<sub>0</sub>` dollars earned are taxed at a rate of `percent<sub>1</sub>`.
// *   The next `upper<sub>2</sub> - upper<sub>1</sub>` dollars earned are taxed at a rate of `percent<sub>2</sub>`.
// *   And so on.
//
// You are given an integer `income` representing the amount of money you earned. Return _the amount of money that you have to pay in taxes._ Answers within `10<sup>-5</sup>` of the actual answer will be accepted.
//
// **Example 1:**
//
// ```
// **Input:** brackets = [[3,50],[7,10],[12,25]], income = 10
// **Output:** 2.65000
// **Explanation:**
// Based on your income, you have 3 dollars in the 1<sup>st</sup> tax bracket, 4 dollars in the 2<sup>nd</sup> tax bracket, and 3 dollars in the 3<sup>rd</sup> tax bracket.
// The tax rate for the three tax brackets is 50%, 10%, and 25%, respectively.
// In total, you pay $3 * 50% + $4 * 10% + $3 * 25% = $2.65 in taxes.
// ```
//
// **Example 2:**
//
// ```
// **Input:** brackets = [[1,0],[4,25],[5,50]], income = 2
// **Output:** 0.25000
// **Explanation:**
// Based on your income, you have 1 dollar in the 1<sup>st</sup> tax bracket and 1 dollar in the 2<sup>nd</sup> tax bracket.
// The tax rate for the two tax brackets is 0% and 25%, respectively.
// In total, you pay $1 * 0% + $1 * 25% = $0.25 in taxes.
// ```
//
// **Example 3:**
//
// ```
// **Input:** brackets = [[2,50]], income = 0
// **Output:** 0.00000
// **Explanation:**
// You have no income to tax, so you have to pay a total of $0 in taxes.
// ```
//
// **Constraints:**
//
// *   `1 <= brackets.length <= 100`
// *   `1 <= upper<sub>i</sub> <= 1000`
// *   `0 <= percent<sub>i</sub> <= 100`
// *   `0 <= income <= 1000`
// *   `upper<sub>i</sub>` is sorted in ascending order.
// *   All the values of `upper<sub>i</sub>` are **unique**.
// *   The upper bound of the last tax bracket is greater than or equal to `income`.

pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
    let mut tax = 0.0;
    let mut prev_upper = 0;
    for bracket in brackets {
        let (upper, percent) = (bracket[0], bracket[1]);
        if income > upper {
            tax += (upper - prev_upper) as f64 * percent as f64 / 100.0;
            prev_upper = upper;
        } else {
            tax += (income - prev_upper) as f64 * percent as f64 / 100.0;
            break;
        }
    }
    return tax;
}

#[test]
pub fn t1() {
    assert_eq!(
        calculate_tax(vec![vec![3, 50], vec![7, 10], vec![12, 25]], 10),
        2.65
    );
}
