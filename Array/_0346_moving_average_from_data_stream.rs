// https://leetcode.com/problems/moving-average-from-data-stream

struct MovingAverage {
    sum: i32,
    size: i32,
    queue: Vec<i32>,
}

impl MovingAverage {
    fn new(size: i32) -> Self {
        MovingAverage {
            sum: 0,
            size,
            queue: Vec::new(),
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.sum += val;
        self.queue.push(val);
        if self.queue.len() > self.size as usize {
            self.sum -= self.queue.remove(0);
        }
        self.sum as f64 / self.queue.len() as f64
    }
}

#[test]
pub fn t1() {
    let mut obj = MovingAverage::new(3);
    assert_eq!(obj.next(1), 1.0);
    assert_eq!(obj.next(2), 1.5);
    assert_eq!(obj.next(3), 2.0);
    assert_eq!(obj.next(4), 3.0);
}
