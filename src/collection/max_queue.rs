
#[derive(Debug,PartialEq,Eq)]
struct MaxQueueItem {
    pub val: i64,
    pub max: i64,
}

#[derive(Debug)]
struct MaxStack(Vec<MaxQueueItem>);

impl MaxStack {
    fn push(&mut self, val: i64) {
        let max = self.get_max().max(val);
        self.0.push(MaxQueueItem{ val, max });
    }
    fn pop(&mut self) -> Option<MaxQueueItem> {
        return self.0.pop();
    }
    fn get_max(&self) -> i64 {
        if self.0.len() == 0 {
            return 0;
        }
        return self.0[self.0.len()-1].max;
    }
    fn len(&self) -> usize {
        return self.0.len();
    }
}

#[derive(Debug)]
pub struct MaxQueue {
    left_stack: MaxStack,
    right_stack: MaxStack,
}

impl MaxQueue {
    pub fn new() -> Self {
        let left_stack = MaxStack(Vec::new());
        let right_stack = MaxStack(Vec::new());
        return Self { left_stack, right_stack };
    }
    pub fn with_capacity(n: usize) -> Self {
        let left_stack = MaxStack(Vec::with_capacity(n));
        let right_stack = MaxStack(Vec::with_capacity(n));
        return Self { left_stack, right_stack };
    }
    pub fn pop(&mut self) -> Option<i64> {
        self.mv();
        return self.left_stack.pop().map(|v| v.val);
    }
    pub fn peek(&mut self) -> Option<i64> {
        self.mv();
        if self.left_stack.len() == 0 {
            return None;
        } else {
            return self.left_stack.0
                .get(self.left_stack.len()-1)
                .map(|v| v.val);
        }
    }
    fn mv(&mut self) {
        if self.left_stack.len() == 0 {
            while let Some(item) = self.right_stack.pop() {
                self.left_stack.push(item.val);
            }
        }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_queue() {
        let mut mq = MaxQueue::with_capacity(5);
        assert_eq!(0, mq.get_max());
        mq.push(3);
        mq.push(2);
        mq.push(5);
        mq.push(1);
        mq.push(4);
        assert_eq!(5, mq.get_max());
        assert_eq!(3, mq.peek().unwrap());
        assert_eq!(3, mq.pop().unwrap());
        assert_eq!(5, mq.get_max());
        assert_eq!(2, mq.pop().unwrap());
        assert_eq!(5, mq.get_max());
        assert_eq!(5, mq.pop().unwrap());
        assert_eq!(4, mq.get_max());
        mq.push(3);
        assert_eq!(4, mq.get_max());
        mq.push(6);
        assert_eq!(6, mq.get_max());
        assert_eq!(1, mq.pop().unwrap());
        assert_eq!(4, mq.pop().unwrap());
        assert_eq!(3, mq.pop().unwrap());
        assert_eq!(6, mq.pop().unwrap());
        assert_eq!(None, mq.pop());
        assert_eq!(0, mq.get_max());
    }
}
