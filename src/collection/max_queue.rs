
struct MaxQueueItem {
    val: i64,
    max: i64,
}

struct MaxStack(Vec<MaxQueueItem>);

impl MaxStack {
    fn push(&mut self, val: i64) {
        let max = self.get_max();
        self.0.push(MaxQueueItem{ val, max });
    }
    fn pop(&mut self) -> Option<MaxQueueItem> {
        return self.0.pop();
    }
    fn get_max(&self) -> i64 {
        return self.0[self.0.len()].max;
    }
    fn len(&self) -> usize {
        return self.0.len();
    }
}

pub struct MaxQueue {
    left_stack: MaxStack,
    right_stack: MaxStack,
}

impl MaxQueue {
    pub fn pop(&mut self) -> i64 {
        if self.left_stack.len() == 0 {
            while let Some(item) = self.right_stack.pop() {
                self.left_stack.push(item.val);
            }
        }
        return 0;
    }
    pub fn push(&mut self, val: i64) {
        self.right_stack.push(val);
    }
    pub fn len(&self) -> usize {
        return self.left_stack.len() + self.right_stack.len();
    }
    pub fn get_max(&self) -> i64 {
        return self.left_stack.get_max()
            .max(self.right_stack.get_max());
    }
}
