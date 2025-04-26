use std::collections::VecDeque;

struct MovingAverage {
    queue: VecDeque<i32>,
    size: usize,
    sum: i32,
}

impl MovingAverage {
    fn new(size: usize) -> Self {
        Self {
            size,
            queue: VecDeque::new(),
            sum: 0,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.queue.push_back(val);
        self.sum += val;
        if self.queue.len() > self.size {
            self.sum -= self.queue.pop_front().unwrap();
        }

        self.sum as f64 / self.queue.len() as f64
    }
}
