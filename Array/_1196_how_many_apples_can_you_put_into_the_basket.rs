// https://leetcode.com/problems/how-many-apples-can-you-put-into-the-basket

pub fn max_number_of_apples(weight: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut sum = 0;
    let mut w = weight.clone();
    w.sort();
    for i in w.iter() {
        sum += i;
        if sum > 5000 {
            break;
        }
        count += 1;
    }
    return count;
}

#[test]
pub fn t1() {
    assert_eq!(max_number_of_apples(vec![100, 200, 150, 1000]), 4);
}
