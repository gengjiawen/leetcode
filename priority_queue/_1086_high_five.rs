// https://leetcode.com/problems/high-five

pub fn high_five(items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut btm: std::collections::BTreeMap<
        i32,
        std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
    > = std::collections::BTreeMap::new();
    for item in items {
        let id = item[0];
        let score = item[1];
        let pq = btm.entry(id).or_default();
        pq.push(std::cmp::Reverse(score));
        if pq.len() > 5 {
            pq.pop();
        }
    }
    let mut res = vec![];
    for (id, pq) in btm {
        res.push(vec![id, pq.iter().map(|v| v.0).sum::<i32>() / 5]);
    }
    return res;
}

#[test]
pub fn t1() {
    let items = vec![
        vec![1, 91],
        vec![1, 92],
        vec![2, 93],
        vec![2, 97],
        vec![1, 60],
        vec![2, 77],
        vec![1, 65],
        vec![1, 87],
        vec![1, 100],
        vec![2, 100],
        vec![2, 76],
    ];
    assert_eq!(high_five(items), vec![vec![1, 87], vec![2, 88]]);
}
