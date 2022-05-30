// https://leetcode.com/problems/defanging-an-ip-address
//
// Given a valid (IPv4) IP `address`, return a defanged version of that IP address.
//
// A _defanged IP address_ replaces every period `"."` with `"[.]"`.
//
// **Example 1:**
//
// ```
// **Input:** address = "1.1.1.1"
// **Output:** "1[.]1[.]1[.]1"
// ```
//
// **Example 2:**
//
// ```
// **Input:** address = "255.100.50.0"
// **Output:** "255[.]100[.]50[.]0"
// ```
//
// **Constraints:**
//
// *   The given `address` is a valid IPv4 address.

pub fn defang_i_paddr(address: String) -> String {
    return address.replace(".", "[.]");
}

#[test]
pub fn t1() {
    assert_eq!(defang_i_paddr("1.1.1.1".to_string()), "1[.]1[.]1[.]1");
}
