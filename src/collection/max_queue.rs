
#[derive(Debug,PartialEq,Eq)]
struct MaxQueueItem<T:Ord+Copy> {
    pub val: T,
    pub max: T,
}

#[derive(Debug)]
struct MaxStack<T:Ord+Copy>(Vec<MaxQueueItem<T>>);

impl <T:Ord+Copy> MaxStack<T> {
    fn push(&mut self, val: T) {
        let max = self.get_max().unwrap_or(val).max(val);
        self.0.push(MaxQueueItem{ val, max });
    }
    fn pop(&mut self) -> Option<MaxQueueItem<T>> {
        return self.0.pop();
    }
    fn get_max(&self) -> Option<T> {
        if self.0.len() == 0 {
            return None;
        }
        return self.0.get(self.0.len()-1).map(|v| v.max);
    }
    fn len(&self) -> usize {
        return self.0.len();
    }
}

#[derive(Debug)]
pub struct MaxQueue<T:Ord+Copy> {
    left_stack: MaxStack<T>,
    right_stack: MaxStack<T>,
}

impl <T:Ord+Copy> MaxQueue<T> {
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
    pub fn pop(&mut self) -> Option<T> {
        self.maybe_move();
        return self.left_stack.pop().map(|v| v.val);
    }
    pub fn peek(&mut self) -> Option<T> {
        self.maybe_move();
        if self.left_stack.len() == 0 {
            return None;
        } else {
            return self.left_stack.0
                .get(self.left_stack.len()-1)
                .map(|v| v.val);
        }
    }
    fn maybe_move(&mut self) {
        if self.left_stack.len() == 0 {
            while let Some(item) = self.right_stack.pop() {
                self.left_stack.push(item.val);
            }
        }
    }
    pub fn push(&mut self, val: T) {
        self.right_stack.push(val);
    }
    pub fn len(&self) -> usize {
        return self.left_stack.len() + self.right_stack.len();
    }
    pub fn get_max(&mut self) -> Option<T> {
        self.maybe_move();
        let left = self.left_stack.get_max();
        if left.is_none() {
            return None;
        }
        let left = left.unwrap();
        return Some(self.right_stack.get_max().unwrap_or(left).max(left));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_queue() {
        let mut mq = MaxQueue::with_capacity(5);
        assert_eq!(None, mq.get_max());
        assert_eq!(None, mq.pop());
        mq.push(3);
        mq.push(2);
        mq.push(5);
        mq.push(1);
        mq.push(4);
        assert_eq!(5, mq.get_max().unwrap());
        assert_eq!(3, mq.peek().unwrap());
        assert_eq!(3, mq.pop().unwrap());
        assert_eq!(5, mq.get_max().unwrap());
        assert_eq!(2, mq.pop().unwrap());
        assert_eq!(5, mq.get_max().unwrap());
        assert_eq!(5, mq.pop().unwrap());
        assert_eq!(4, mq.get_max().unwrap());
        mq.push(3);
        assert_eq!(4, mq.get_max().unwrap());
        mq.push(6);
        assert_eq!(6, mq.get_max().unwrap());
        assert_eq!(1, mq.pop().unwrap());
        assert_eq!(4, mq.pop().unwrap());
        assert_eq!(3, mq.pop().unwrap());
        assert_eq!(6, mq.pop().unwrap());
        assert_eq!(None, mq.pop());
        assert_eq!(None, mq.get_max());
    }
}
