// https://leetcode.com/problems/meeting-rooms

pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
    if intervals.len() < 2 {
        return true;
    }

    let mut intervals = intervals;
    intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
    for i in 0..intervals.len() - 1 {
        if intervals[i][1] > intervals[i + 1][0] {
            return false;
        }
    }
    return true;
}

#[test]
pub fn t1() {
    assert_eq!(
        can_attend_meetings(vec![vec![0, 30], vec![5, 10], vec![15, 20]]),
        false
    );
    assert_eq!(can_attend_meetings(vec![vec![7, 10], vec![2, 4]]), true);
}
