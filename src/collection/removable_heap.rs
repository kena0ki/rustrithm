use std::collections::{BinaryHeap, binary_heap::PeekMut};

pub struct RemovableHeap<T> {
    pub heap: BinaryHeap<T>,
    pub del: BinaryHeap<T>,
}

impl <T:Ord> RemovableHeap<T> {
    pub fn new() -> Self {
        return Self::with_capacity(0);
    }
    pub fn with_capacity(n: usize) -> Self {
        let heap= BinaryHeap::with_capacity(n);
        let del = BinaryHeap::with_capacity(n);
        return Self { heap, del };
    }
    pub fn push(&mut self, v:T) {
        self.heap.push(v);
    }
    pub fn remove(&mut self, v:T) {
        self.del.push(v);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.cleanup();
        return self.heap.pop();
    }
    pub fn peek(&mut self) -> Option<&T> {
        self.cleanup();
        return self.heap.peek();
    }
    pub fn peek_mut(&mut self) -> Option<PeekMut<'_, T>> {
        self.cleanup();
        return self.heap.peek_mut();
    }
    fn cleanup(&mut self) {
        while self.heap.peek().is_some() && self.heap.peek() == self.del.peek() {
            self.heap.pop();
            self.del.pop();
        }
    }
    pub fn clear(&mut self) {
        self.heap.clear();
        self.del.clear();
    }
}

impl <T:Ord> From<Vec<T>> for RemovableHeap<T> {
    fn from(v: Vec<T>) -> Self {
        let del = BinaryHeap::with_capacity(v.len());
        let heap = BinaryHeap::from(v);
        return Self { heap, del };
    }
}

#[cfg(test)]
mod test{
    use super::RemovableHeap;

    #[test]
    fn test_removable_heap() {
        let mut rh = RemovableHeap::from(vec![1,1,2,3,4,5]);
        rh.push(4);
        rh.push(0);
        assert_eq!(Some(&5),rh.peek());
        rh.remove(3);
        rh.remove(2);
        rh.remove(1);
        assert_eq!(Some(5),rh.pop());
        assert_eq!(Some(4),rh.pop());
        assert_eq!(Some(&4),rh.peek());
        assert_eq!(Some(4),rh.pop());
        assert_eq!(Some(1),rh.pop());
        assert_eq!(Some(0),rh.pop());
        assert_eq!(None,rh.peek());
        assert_eq!(None,rh.pop());
    }
}
