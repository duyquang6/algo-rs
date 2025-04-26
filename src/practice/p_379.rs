struct PhoneDirectory {
    used: Vec<bool>,
    avail_queue: Vec<i32>,
    head: usize,
    tail: usize,
    size: usize,
}

impl PhoneDirectory {
    fn new(max_numbers: i32) -> Self {
        Self {
            head: 0,
            tail: max_numbers as usize,
            avail_queue: vec![0i32; max_numbers as usize],
            used: vec![false; max_numbers as usize],
            size: max_numbers as usize,
        }
    }

    fn get(&mut self) -> i32 {
        if self.tail - self.head == self.size {
            return -1;
        }
        let ans = self.avail_queue[self.head % self.size];
        self.head += 1;
        ans as i32
    }

    fn check(&self, number: i32) -> bool {
        self.used[number as usize]
    }

    fn release(&mut self, number: i32) {
        if !self.used[number as usize] {
            return;
        }
        self.avail_queue[self.tail % self.size] = number;
        self.tail += 1;
    }
}
