struct MinStack {
    stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack { stack: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        let mut sorted = self.stack.to_owned();

        sorted.last().unwrap().to_owned()
    }

    fn get_min(&self) -> i32 {
        let mut sorted = self.stack.to_owned();
        sorted.sort_unstable();

        sorted.first().unwrap().to_owned()
    }
}
