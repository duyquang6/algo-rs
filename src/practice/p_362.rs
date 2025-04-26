struct HitCounter {
    queue: Vec<u64>,
    head: usize,
    tail: usize,
    total: i32,
}

impl HitCounter {
    fn new() -> Self {
        Self {
            queue: vec![0; 300],
            head: 0,
            tail: 0,
            total: 0,
        }
    }

    fn hit(&mut self, timestamp: i32) {
        if self.tail == 0 {
            self.queue[self.tail % 300] = timestamp as u64 * 1000 + 1;
            self.tail += 1;
        } else {
            let tail_timestamp = self.queue[(self.tail - 1) % 300] / 1000;
            if tail_timestamp == timestamp as u64 {
                self.queue[(self.tail - 1) % 300] += 1;
            } else {
                self.queue[self.tail % 300] = timestamp as u64 * 1000 + 1;
                self.tail += 1;
            }
        }
        self.total += 1;
    }

    fn get_hits(&mut self, timestamp: i32) -> i32 {
        let start_timestamp = timestamp - 300;

        while self.head < self.tail {
            let head_item = self.queue[self.head % 300];
            let head_timestamp = head_item / 1000;
            if head_timestamp < start_timestamp as u64 {
                // pop
                self.total -= (head_item % 1000) as i32;
                self.head += 1;
            } else {
                break;
            }
        }

        self.total
    }
}
