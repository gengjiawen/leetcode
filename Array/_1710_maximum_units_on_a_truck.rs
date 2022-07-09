// https://leetcode.com/problems/maximum-units-on-a-truck
//
// You are assigned to put some amount of boxes onto **one truck**. You are given a 2D array `boxTypes`, where `boxTypes[i] = [numberOfBoxes<sub>i</sub>, numberOfUnitsPerBox<sub>i</sub>]`:
//
// *   `numberOfBoxes<sub>i</sub>` is the number of boxes of type `i`.
// *   `numberOfUnitsPerBox<sub>i</sub>`is the number of units in each box of the type `i`.
//
// You are also given an integer `truckSize`, which is the **maximum** number of **boxes** that can be put on the truck. You can choose any boxes to put on the truck as long as the number of boxes does not exceed `truckSize`.
//
// Return _the **maximum** total number of **units** that can be put on the truck._
//
// **Example 1:**
//
// ```
// **Input:** boxTypes = [[1,3],[2,2],[3,1]], truckSize = 4
// **Output:** 8
// **Explanation:** There are:
// - 1 box of the first type that contains 3 units.
// - 2 boxes of the second type that contain 2 units each.
// - 3 boxes of the third type that contain 1 unit each.
// You can take all the boxes of the first and second types, and one box of the third type.
// The total number of units will be = (1 * 3) + (2 * 2) + (1 * 1) = 8.
// ```
//
// **Example 2:**
//
// ```
// **Input:** boxTypes = [[5,10],[2,5],[4,7],[3,9]], truckSize = 10
// **Output:** 91
// ```
//
// **Constraints:**
//
// *   `1 <= boxTypes.length <= 1000`
// *   `1 <= numberOfBoxes<sub>i</sub>, numberOfUnitsPerBox<sub>i</sub> <= 1000`
// *   `1 <= truckSize <= 10<sup>6</sup>`

pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
    let mut sorted_box = box_types.clone();
    sorted_box.sort_unstable_by_key(|x| x[1]);
    let boxes = sorted_box
        .iter()
        .map(|x| vec![x[0]; x[1] as usize])
        .flatten()
        .take(truck_size as usize)
        .collect::<Vec<i32>>();

    return boxes.into_iter().sum();
}

#[test]
pub fn t1() {
    assert_eq!(
        maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4),
        8
    );
    assert_eq!(
        maximum_units(vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]], 4),
        8
    );
}
